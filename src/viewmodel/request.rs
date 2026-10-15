//! View-model backing the request editor.

use crate::model::{KVKind, KeyValue, ReqTab, urlencode};

/// View-model for the request section. Fields are private; the view may only
/// read through getters and mutate through commands, which keeps invariants
/// (uppercased method, valid row indices) enforced in one place.
pub struct RequestVm {
    method: String,
    url: String,
    params: Vec<KeyValue>,
    headers: Vec<KeyValue>,
    body: String,
    tab: ReqTab,
    sending: bool,
}

impl Default for RequestVm {
    fn default() -> Self {
        Self {
            method: "GET".to_string(),
            url: String::new(),
            params: vec![KeyValue::default()],
            headers: vec![KeyValue::default()],
            body: String::new(),
            tab: ReqTab::Params,
            sending: false,
        }
    }
}

impl RequestVm {
    // -- Reads -------------------------------------------------------------

    pub fn method(&self) -> &str {
        &self.method
    }

    pub fn url(&self) -> &str {
        &self.url
    }

    pub fn body(&self) -> &str {
        &self.body
    }

    pub fn tab(&self) -> ReqTab {
        self.tab
    }

    pub fn sending(&self) -> bool {
        self.sending
    }

    /// Borrow the key/value list selected by `kind`.
    pub fn kv(&self, kind: KVKind) -> &[KeyValue] {
        match kind {
            KVKind::Params => &self.params,
            KVKind::Headers => &self.headers,
        }
    }

    /// Build the final URL, merging in any non-empty query params.
    pub fn build_url(&self) -> String {
        let mut url = self.url.trim().to_string();
        let query: Vec<String> = self
            .params
            .iter()
            .filter(|p| !p.key.trim().is_empty())
            .map(|p| format!("{}={}", urlencode(p.key.trim()), urlencode(p.value.trim())))
            .collect();
        if !query.is_empty() {
            let sep = if url.contains('?') { '&' } else { '?' };
            url = format!("{url}{sep}{}", query.join("&"));
        }
        url
    }

    /// Headers with a blank key are dropped.
    pub fn active_headers(&self) -> Vec<(String, String)> {
        self.headers
            .iter()
            .filter(|h| !h.key.trim().is_empty())
            .map(|h| (h.key.trim().to_string(), h.value.clone()))
            .collect()
    }

    // -- Commands ----------------------------------------------------------

    /// Methods are stored uppercased.
    pub fn set_method(&mut self, value: String) {
        self.method = value.to_uppercase();
    }

    pub fn set_url(&mut self, value: String) {
        self.url = value;
    }

    pub fn set_body(&mut self, value: String) {
        self.body = value;
    }

    /// Replace the header rows, keeping a single blank row when `headers` is empty.
    pub fn set_headers(&mut self, headers: Vec<(String, String)>) {
        self.headers = if headers.is_empty() {
            vec![KeyValue::default()]
        } else {
            headers
                .into_iter()
                .map(|(key, value)| KeyValue { key, value })
                .collect()
        };
    }

    pub fn set_tab(&mut self, tab: ReqTab) {
        self.tab = tab;
    }

    pub fn set_sending(&mut self, sending: bool) {
        self.sending = sending;
    }

    pub fn add_kv(&mut self, kind: KVKind) {
        self.kv_mut(kind).push(KeyValue::default());
    }

    pub fn remove_kv(&mut self, kind: KVKind, index: usize) {
        let list = self.kv_mut(kind);
        if index < list.len() {
            list.remove(index);
        }
    }

    pub fn set_kv_key(&mut self, kind: KVKind, index: usize, value: String) {
        if let Some(kv) = self.kv_mut(kind).get_mut(index) {
            kv.key = value;
        }
    }

    pub fn set_kv_value(&mut self, kind: KVKind, index: usize, value: String) {
        if let Some(kv) = self.kv_mut(kind).get_mut(index) {
            kv.value = value;
        }
    }

    fn kv_mut(&mut self, kind: KVKind) -> &mut Vec<KeyValue> {
        match kind {
            KVKind::Params => &mut self.params,
            KVKind::Headers => &mut self.headers,
        }
    }
}
