//! View-model layer.
//!
//! Each screen region has its own view-model and [`App`] composes them into the
//! root view-model. Larger view-models own smaller ones: [`App`] owns the
//! [`WorkspaceVm`], which owns a [`TabVm`] per open document, and each tab owns
//! a [`RequestVm`] and a [`ResponseVm`]. Every user event is handled by a
//! command method on a view-model; widgets never mutate state directly and
//! never talk to the network themselves.

pub mod collections;
mod history;
mod request;
mod response;
mod tab;
mod workspace;

pub use collections::CollectionsVm;
pub use history::HistoryVm;
pub use tab::{TabSource, TabVm};
pub use workspace::WorkspaceVm;

use crate::model::{HistoryEntry, RequestOutcome, RequestTask};

/// Root view-model. Owns the sub view-models and the worker channel.
pub struct App {
    pub workspace: WorkspaceVm,
    pub history: HistoryVm,
    pub collections: CollectionsVm,
    sender: Option<tokio::sync::mpsc::UnboundedSender<RequestTask>>,
}

impl App {
    pub fn new() -> Self {
        Self {
            workspace: WorkspaceVm::default(),
            history: HistoryVm::default(),
            collections: CollectionsVm::new_with_examples(),
            sender: None,
        }
    }

    // -- Worker lifecycle --------------------------------------------------

    /// Called once by the worker so the view-model can dispatch requests.
    pub fn attach_sender(&mut self, sender: tokio::sync::mpsc::UnboundedSender<RequestTask>) {
        self.sender = Some(sender);
    }

    /// Record a completed request coming back from the worker.
    pub fn on_response(&mut self, outcome: RequestOutcome) {
        self.workspace
            .set_response(outcome.tab_id, outcome.response);
    }

    // -- Commands ----------------------------------------------------------

    /// Validate the active request, record it in history and dispatch it.
    pub fn send(&mut self) {
        let Some(tab_id) = self.workspace.active_id() else {
            return;
        };

        let (method, url, headers, body) = {
            let Some(tab) = self.workspace.active() else {
                return;
            };
            let url = tab.request().build_url();
            if url.is_empty() {
                return;
            }
            let method = {
                let method = tab.request().method().trim().to_uppercase();
                if method.is_empty() {
                    "GET".to_string()
                } else {
                    method
                }
            };
            (
                method,
                url,
                tab.request().active_headers(),
                tab.request().body().to_string(),
            )
        };

        let task = RequestTask {
            tab_id,
            method: method.clone(),
            url: url.clone(),
            headers,
            body,
        };

        self.workspace.set_sending(true);
        self.workspace.clear_response();
        self.history.insert(HistoryEntry { method, url });
        match &self.sender {
            Some(sender) => {
                let _ = sender.send(task);
            }
            None => self.workspace.set_sending(false),
        }

        self.save_tab(tab_id);
    }

    /// Open a collection request in the workspace, reusing an existing tab.
    pub fn open_request(&mut self, req_id: u64) {
        let Some(item) = self.collections.find_request(req_id) else {
            return;
        };
        self.workspace.open_request(item);
        self.collections.set_selected(req_id);
    }

    /// Reload a past request into a fresh tab.
    pub fn select_history(&mut self, index: usize) {
        let Some(entry) = self.history.entries().get(index) else {
            return;
        };
        let method = entry.method.clone();
        let url = entry.url.clone();
        let title = crate::util::truncate(&url, 24);
        let id = self.workspace.open_scratch();
        self.workspace.set_method(method);
        self.workspace.set_url(url);
        self.workspace.set_title(id, title);
    }

    /// Persist the active tab's editor state back into its collection request.
    fn save_tab(&mut self, tab_id: u64) {
        let request_id = {
            let Some(tab) = self.workspace.tab(tab_id) else {
                return;
            };
            match tab.source() {
                TabSource::CollectionRequest { request_id } => *request_id,
                TabSource::Scratch => return,
            }
        };
        if let (Some(item), Some(tab)) = (
            self.collections.find_request_mut(request_id),
            self.workspace.tab(tab_id),
        ) {
            item.update_from_request(tab.request());
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn save_tab_writes_edits_back_to_collection() {
        let mut app = App::new();
        let req_id = app.collections.collections()[0].requests()[0].id();

        app.open_request(req_id);
        app.workspace
            .set_url("https://example.org/changed".to_string());

        app.send();

        let item = app.collections.find_request(req_id).unwrap();
        assert_eq!(item.url(), "https://example.org/changed");
    }

    #[test]
    fn open_request_selects_the_leaf() {
        let mut app = App::new();
        let req_id = app.collections.collections()[0].requests()[0].id();

        app.open_request(req_id);

        assert_eq!(app.workspace.active().map(|t| t.title()), Some("quick get"));
        assert!(app.collections.find_request(req_id).unwrap().is_selected);
    }
}
