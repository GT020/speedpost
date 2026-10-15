use xilem::masonry::properties::types::AsUnit;
use xilem::style::Style as _;
use xilem::view::{flex_col, label, prose};
use xilem::{AnyWidgetView, WidgetView};

use super::expander::{GuideLineConfig, HeaderStyle, chevron, expander_with_line};
use super::request_item;
use crate::theme;
use crate::viewmodel::App;
use crate::viewmodel::collections::FolderVm;

/// Recursive folder expander widget with very fine guide line and stable layout.
pub fn view(folder: &FolderVm) -> Box<AnyWidgetView<App>> {
    let folder_id = folder.id();
    let expanded = folder.is_expanded();

    let mut children: Vec<Box<AnyWidgetView<App>>> = Vec::new();
    if expanded {
        // 1. Recursive sub-folders
        for sub in folder.folders() {
            children.push(view(sub));
        }

        // 2. Leaf requests
        for req in folder.requests() {
            children.push(request_item::view(req).boxed());
        }

        // 3. Subtle empty state if folder has no items
        if folder.folders().is_empty() && folder.requests().is_empty() {
            let empty_hint = prose("Folder is empty")
                .text_size(10.0)
                .text_color(theme::RED);
            children.push(empty_hint.boxed());
        }
    }

    let indicator = chevron(expanded, theme::RED);
    let header_label = label(folder.name()).text_size(13.0).color(theme::MUTED);

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
            st.collections.toggle_folder(folder_id);
        },
        move |st: &mut App| {
            st.collections.toggle_folder(folder_id);
        },
        content_col,
    )
    .boxed()
}
