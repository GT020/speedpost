//! View-model backing the response viewer.

use crate::model::{HttpResponse, RespTab};

/// View-model for the response section. Private state, getters for reads and
/// commands for writes.
pub struct ResponseVm {
    tab: RespTab,
    response: Option<HttpResponse>,
}

impl Default for ResponseVm {
    fn default() -> Self {
        Self {
            tab: RespTab::Body,
            response: None,
        }
    }
}

impl ResponseVm {
    // -- Reads -------------------------------------------------------------

    pub fn tab(&self) -> RespTab {
        self.tab
    }

    pub fn response(&self) -> Option<&HttpResponse> {
        self.response.as_ref()
    }

    // -- Commands ----------------------------------------------------------

    pub fn set_tab(&mut self, tab: RespTab) {
        self.tab = tab;
    }

    pub fn set_response(&mut self, response: HttpResponse) {
        self.response = Some(response);
    }

    pub fn clear(&mut self) {
        self.response = None;
    }
}
