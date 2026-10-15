//! Main workspace: a tab bar over the request editor and response viewer.

use xilem::core::one_of::OneOf2;
use xilem::style::Style as _;
use xilem::view::{Axis, flex_col, prose, split};
use xilem::{Color, WidgetView};

use crate::theme;
use crate::viewmodel::App;
use crate::widgets::{
    request_bar, request_editor, request_tabs, response_body, response_status, response_tabs,
    tab_bar,
};

pub fn view(state: &App) -> impl WidgetView<App> + use<> {
    let body = match state.workspace.active() {
        Some(tab) => {
            let request_pane = flex_col((request_bar(tab), request_tabs(tab), request_editor(tab)))
                .must_fill_major_axis(true)
                .background_color(Color::from_rgb8(0xFF, 0x6C, 0x37));

            let response_pane =
                flex_col((response_status(tab), response_tabs(tab), response_body(tab)))
                    .must_fill_major_axis(true);

            OneOf2::A(
                split(request_pane, response_pane)
                    .split_axis(Axis::Vertical)
                    .split_point(0.5),
            )
        }
        None => OneOf2::B(
            flex_col((
                prose("No open tabs — open a request from Collections or press +.")
                    .text_size(14.0)
                    .text_color(theme::MUTED),
            ))
            .padding(16.0),
        ),
    };

    flex_col((tab_bar(state), body))
        .must_fill_major_axis(true)
        .cross_axis_alignment(xilem::view::CrossAxisAlignment::Start)
}
