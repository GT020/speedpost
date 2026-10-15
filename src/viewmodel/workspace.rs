//! View-model for the workspace: the collection of open tabs and which one is active.
//!
//! This is the larger view-model that owns the smaller [`TabVm`] documents, each
//! of which in turn owns a request and a response view-model.

use super::tab::TabVm;
use crate::model::{HttpResponse, KVKind, ReqTab, RespTab};
use crate::viewmodel::collections::RequestItemVm;

/// Root view-model for the tabbed workspace.
#[derive(Default)]
pub struct WorkspaceVm {
    tabs: Vec<TabVm>,
    active: Option<u64>,
    next_id: u64,
}

impl WorkspaceVm {
    // -- Reads -------------------------------------------------------------

    pub fn tabs(&self) -> &[TabVm] {
        &self.tabs
    }

    pub fn active_id(&self) -> Option<u64> {
        self.active
    }

    pub fn active(&self) -> Option<&TabVm> {
        let id = self.active?;
        self.tab(id)
    }

    pub fn active_mut(&mut self) -> Option<&mut TabVm> {
        let id = self.active?;
        let idx = self.tabs.iter().position(|t| t.id() == id)?;
        self.tabs.get_mut(idx)
    }

    pub fn tab(&self, id: u64) -> Option<&TabVm> {
        self.tabs.iter().find(|t| t.id() == id)
    }

    // -- Tab lifecycle -----------------------------------------------------

    fn alloc_id(&mut self) -> u64 {
        self.next_id += 1;
        self.next_id
    }

    /// Open a collection request, reusing an existing tab for it when present.
    /// Returns the id of the (now active) tab.
    pub fn open_request(&mut self, item: &RequestItemVm) -> u64 {
        if let Some(tab) = self.tabs.iter().find(|t| t.matches_request(item.id)) {
            let id = tab.id();
            self.active = Some(id);
            return id;
        }
        let id = self.alloc_id();
        self.tabs.push(TabVm::from_request_item(id, item));
        self.active = Some(id);
        id
    }

    /// Open a blank scratch tab and activate it.
    pub fn open_scratch(&mut self) -> u64 {
        let id = self.alloc_id();
        self.tabs.push(TabVm::from_scratch(id));
        self.active = Some(id);
        id
    }

    /// Close a tab, activating a neighbour when the closed tab was active.
    pub fn close(&mut self, id: u64) {
        let Some(idx) = self.tabs.iter().position(|t| t.id() == id) else {
            return;
        };
        self.tabs.remove(idx);

        if self.active != Some(id) {
            return;
        }
        if self.tabs.is_empty() {
            self.active = None;
        } else {
            let next = idx.min(self.tabs.len() - 1);
            self.active = Some(self.tabs[next].id());
        }
    }

    pub fn activate(&mut self, id: u64) {
        if self.tabs.iter().any(|t| t.id() == id) {
            self.active = Some(id);
        }
    }

    pub fn set_title(&mut self, id: u64, title: String) {
        if let Some(tab) = self.tabs.iter_mut().find(|t| t.id() == id) {
            tab.set_title(title);
        }
    }

    // -- Delegations to the active tab -------------------------------------
    //
    // Widgets only ever render the active tab, so their event closures can
    // target it through these thin commands instead of threading a tab id.

    pub fn set_method(&mut self, value: String) {
        if let Some(tab) = self.active_mut() {
            tab.request_mut().set_method(value);
        }
    }

    pub fn set_url(&mut self, value: String) {
        if let Some(tab) = self.active_mut() {
            tab.request_mut().set_url(value);
        }
    }

    pub fn set_body(&mut self, value: String) {
        if let Some(tab) = self.active_mut() {
            tab.request_mut().set_body(value);
        }
    }

    pub fn set_tab(&mut self, tab: ReqTab) {
        if let Some(active) = self.active_mut() {
            active.request_mut().set_tab(tab);
        }
    }

    pub fn set_response_tab(&mut self, tab: RespTab) {
        if let Some(active) = self.active_mut() {
            active.response_mut().set_tab(tab);
        }
    }

    pub fn add_kv(&mut self, kind: KVKind) {
        if let Some(active) = self.active_mut() {
            active.request_mut().add_kv(kind);
        }
    }

    pub fn remove_kv(&mut self, kind: KVKind, index: usize) {
        if let Some(active) = self.active_mut() {
            active.request_mut().remove_kv(kind, index);
        }
    }

    pub fn set_kv_key(&mut self, kind: KVKind, index: usize, value: String) {
        if let Some(active) = self.active_mut() {
            active.request_mut().set_kv_key(kind, index, value);
        }
    }

    pub fn set_kv_value(&mut self, kind: KVKind, index: usize, value: String) {
        if let Some(active) = self.active_mut() {
            active.request_mut().set_kv_value(kind, index, value);
        }
    }

    pub fn set_sending(&mut self, sending: bool) {
        if let Some(active) = self.active_mut() {
            active.request_mut().set_sending(sending);
        }
    }

    pub fn clear_response(&mut self) {
        if let Some(active) = self.active_mut() {
            active.response_mut().clear();
        }
    }

    /// Route a completed response to the tab that dispatched it.
    pub fn set_response(&mut self, tab_id: u64, response: HttpResponse) {
        if let Some(tab) = self.tabs.iter_mut().find(|t| t.id() == tab_id) {
            tab.request_mut().set_sending(false);
            tab.response_mut().set_response(response);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::RequestItem;
    use crate::viewmodel::collections::RequestItemVm;

    fn item(id: u64, name: &str) -> RequestItemVm {
        RequestItemVm::new(
            id,
            RequestItem {
                name: name.to_string(),
                method: "GET".to_string(),
                url: "https://example.com".to_string(),
                headers: vec![],
                body: String::new(),
            },
        )
    }

    #[test]
    fn open_request_dedupes_and_activates() {
        let mut ws = WorkspaceVm::default();
        let a = item(1, "a");
        let first = ws.open_request(&a);
        assert_eq!(ws.tabs().len(), 1);
        assert_eq!(ws.active_id(), Some(first));

        let again = ws.open_request(&a);
        assert_eq!(first, again);
        assert_eq!(ws.tabs().len(), 1);
    }

    #[test]
    fn closing_active_activates_neighbour() {
        let mut ws = WorkspaceVm::default();
        let a = ws.open_request(&item(1, "a"));
        let b = ws.open_request(&item(2, "b"));
        let c = ws.open_request(&item(3, "c"));
        assert_eq!(ws.active_id(), Some(c));

        ws.close(b);
        assert_eq!(ws.active_id(), Some(c));

        ws.close(c);
        assert_eq!(ws.active_id(), Some(a));
        assert_eq!(ws.tabs().len(), 1);

        ws.close(a);
        assert_eq!(ws.active_id(), None);
        assert!(ws.tabs().is_empty());
    }
}
