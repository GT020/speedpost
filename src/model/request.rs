//! Domain types for building requests and inspecting history.

/// Which request editor tab is active.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum ReqTab {
    Params,
    Headers,
    Body,
}

/// Which response viewer tab is active.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum RespTab {
    Body,
    Headers,
}

/// Selects between the two key/value collections on a request.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum KVKind {
    Params,
    Headers,
}

/// A single editable key/value row, used for both query params and headers.
#[derive(Clone, Default, Debug)]
pub struct KeyValue {
    pub key: String,
    pub value: String,
}

/// A previously sent request, shown in the sidebar history.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct HistoryEntry {
    pub method: String,
    pub url: String,
}
