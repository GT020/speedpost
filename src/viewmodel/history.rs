//! View-model backing the sidebar history.

use crate::model::HistoryEntry;

/// View-model for previously sent requests.
#[derive(Default)]
pub struct HistoryVm {
    entries: Vec<HistoryEntry>,
}

impl HistoryVm {
    // -- Reads -------------------------------------------------------------

    pub fn entries(&self) -> &[HistoryEntry] {
        &self.entries
    }

    // -- Commands ----------------------------------------------------------

    /// Record a request at the top of the list.
    pub fn insert(&mut self, entry: HistoryEntry) {
        self.entries.insert(0, entry);
    }
}
