//! View layer: every screen region is its own self-contained widget module.
//!
//! Each module exposes a `view` function that renders that part against the
//! root [`App`] view-model.

mod body_editor;
mod hover;
mod kv_editor;
mod request_bar;
mod request_editor;
mod request_tabs;
mod response_body;
mod response_status;
mod response_tabs;
mod sidebar;
mod tab_bar;
mod title_bar;
mod workspace;

pub use body_editor::view as body_editor;
pub use kv_editor::view as kv_editor;
pub use request_bar::view as request_bar;
pub use request_editor::view as request_editor;
pub use request_tabs::view as request_tabs;
pub use response_body::view as response_body;
pub use response_status::view as response_status;
pub use response_tabs::view as response_tabs;
pub use sidebar::view as sidebar;
pub use tab_bar::view as tab_bar;
pub use title_bar::view as title_bar;
pub use workspace::view as workspace;

pub mod collection_item;
pub mod expander;
pub mod folder_item;
pub mod request_item;

pub use collection_item::view as collection_item;
#[allow(unused_imports)]
pub use expander::{
    GuideLineConfig, HeaderStyle, chevron, chevron_indicator, expander, expander_header,
    expander_styled, expander_with, expander_with_header_and_line, expander_with_line, guide_line,
    plus_minus_indicator, triangle_indicator,
};
pub use folder_item::view as folder_item;
pub use request_item::view as request_item;
