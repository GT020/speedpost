#![allow(dead_code)]

use crate::model::RequestItem;
use crate::viewmodel::request::RequestVm;

/// Smallest granular view model representing an individual request leaf node.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RequestItemVm {
    pub id: u64,
    pub name: String,
    pub method: String,
    pub url: String,
    pub headers: Vec<(String, String)>,
    pub body: String,
    /// Whether this leaf is the currently selected request in the tree.
    pub is_selected: bool,
}

impl RequestItemVm {
    pub fn new(id: u64, item: RequestItem) -> Self {
        Self {
            id,
            name: item.name,
            method: item.method,
            url: item.url,
            headers: item.headers,
            body: item.body,
            is_selected: false,
        }
    }

    /// Build a request leaf with a generated id and no headers/body.
    pub fn blank(id: u64, name: String, method: String, url: String) -> Self {
        Self {
            id,
            name,
            method,
            url,
            headers: Vec::new(),
            body: String::new(),
            is_selected: false,
        }
    }

    // -- Reads -------------------------------------------------------------

    pub fn id(&self) -> u64 {
        self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn method(&self) -> &str {
        &self.method
    }

    pub fn url(&self) -> &str {
        &self.url
    }

    // -- Commands ----------------------------------------------------------

    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }

    pub fn set_method(&mut self, method: String) {
        self.method = method;
    }

    pub fn set_url(&mut self, url: String) {
        self.url = url;
    }

    pub fn set_headers(&mut self, headers: Vec<(String, String)>) {
        self.headers = headers;
    }

    pub fn set_body(&mut self, body: String) {
        self.body = body;
    }

    pub fn set_selected(&mut self, selected: bool) {
        self.is_selected = selected;
    }

    /// Copy editable fields from an open tab's request editor back into this node.
    pub fn update_from_request(&mut self, request: &RequestVm) {
        self.method = request.method().to_string();
        self.url = request.build_url();
        self.headers = request.active_headers();
        self.body = request.body().to_string();
    }

    pub fn to_model(&self) -> RequestItem {
        RequestItem {
            name: self.name.clone(),
            method: self.method.clone(),
            url: self.url.clone(),
            headers: self.headers.clone(),
            body: self.body.clone(),
        }
    }
}
