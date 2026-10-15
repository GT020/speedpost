//! View-model for a single open request document (a tab).

use super::request::RequestVm;
use super::response::ResponseVm;
use crate::viewmodel::collections::RequestItemVm;

/// Where a tab originated, used to link back into the collection tree.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum TabSource {
    /// A blank tab created with the `+` button or from history.
    Scratch,
    /// Linked to a request leaf in the collection tree.
    CollectionRequest { request_id: u64 },
}

/// One open document. This is a mid-sized view-model that owns the smaller
/// [`RequestVm`] and [`ResponseVm`] as children.
pub struct TabVm {
    id: u64,
    title: String,
    source: TabSource,
    request: RequestVm,
    response: ResponseVm,
}

impl TabVm {
    /// Build a tab seeded from a collection request leaf.
    pub fn from_request_item(id: u64, item: &RequestItemVm) -> Self {
        let mut request = RequestVm::default();
        request.set_method(item.method.clone());
        request.set_url(item.url.clone());
        request.set_headers(item.headers.clone());
        request.set_body(item.body.clone());

        Self {
            id,
            title: item.name.clone(),
            source: TabSource::CollectionRequest {
                request_id: item.id,
            },
            request,
            response: ResponseVm::default(),
        }
    }

    /// Build a blank tab.
    pub fn from_scratch(id: u64) -> Self {
        Self {
            id,
            title: "Untitled".to_string(),
            source: TabSource::Scratch,
            request: RequestVm::default(),
            response: ResponseVm::default(),
        }
    }

    // -- Reads -------------------------------------------------------------

    pub fn id(&self) -> u64 {
        self.id
    }

    pub fn title(&self) -> &str {
        &self.title
    }

    pub fn source(&self) -> &TabSource {
        &self.source
    }

    pub fn request(&self) -> &RequestVm {
        &self.request
    }

    pub fn request_mut(&mut self) -> &mut RequestVm {
        &mut self.request
    }

    pub fn response(&self) -> &ResponseVm {
        &self.response
    }

    pub fn response_mut(&mut self) -> &mut ResponseVm {
        &mut self.response
    }

    /// Whether this tab is linked to the collection request `req_id`.
    pub fn matches_request(&self, req_id: u64) -> bool {
        matches!(
            self.source,
            TabSource::CollectionRequest { request_id } if request_id == req_id
        )
    }

    // -- Commands ----------------------------------------------------------

    pub fn set_title(&mut self, title: String) {
        self.title = title;
    }
}
