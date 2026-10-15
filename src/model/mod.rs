//! Model layer: pure domain types and side-effectful services.
//!
//! Nothing here knows about the UI or about Xilem. The view-model composes
//! these types and the widgets bind to the view-model.

pub mod http;
pub mod request;

pub use http::{HttpResponse, RequestOutcome, RequestTask, perform_request, urlencode};
pub use request::{HistoryEntry, KVKind, KeyValue, ReqTab, RespTab};
pub mod collection;
pub use collection::{Collection, Collections, Folder, RequestItem};
pub mod settings;
pub use settings::Settings;
