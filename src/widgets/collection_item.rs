use xilem::masonry::properties::types::AsUnit;
use xilem::style::Style as _;
use xilem::view::{flex_col, label};
use xilem::{AnyWidgetView, WidgetView};

use super::expander::{GuideLineConfig, HeaderStyle, chevron, expander_with_line};
use super::{folder_item, request_item};
use crate::theme;
use crate::viewmodel::App;
use crate::viewmodel::collections::CollectionVm;

/// Renders a top-level collection expander item with customizable indicator and fine guide line.
pub fn view(coll: &CollectionVm) -> impl WidgetView<App> + use<> {
    let coll_id = coll.id();
    let expanded = coll.is_expanded();

    let mut children: Vec<Box<AnyWidgetView<App>>> = Vec::new();
    if expanded {
        for folder in coll.folders() {
            children.push(folder_item::view(folder));
        }

        for req in coll.requests() {
            children.push(request_item::view(req).boxed());
        }
    }

    let indicator = chevron(expanded, theme::RED);
    let header_label = label(coll.name()).text_size(13.0).color(theme::MUTED);

    let line_config = GuideLineConfig::new()
        .color(theme::GREEN)
        .padding_before(7.0)
        .padding_after(6.0)
        .child_gap(2.0);

    let content_col = flex_col(children)
        .gap(line_config.child_gap.px())
        .cross_axis_alignment(xilem::view::CrossAxisAlignment::Start);

    let header_style = HeaderStyle::default().padding_h(0.0).indicator_gap(10.0);

    expander_with_line(
        indicator,
        header_label,
        expanded,
        true, // opt-in fine guide line
        Some(line_config),
        header_style,
        move |st: &mut App| {
            st.collections.toggle_collection(coll_id);
        },
        move |st: &mut App| {
            st.collections.toggle_collection(coll_id);
        },
        content_col,
    )
}
