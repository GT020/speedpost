//! Raw request body editor.

use xilem::masonry::properties::types::AsUnit;
use xilem::style::Style as _;
use xilem::view::{FlexSpacer, flex_col, portal, prose, text_input};
use xilem::{InsertNewline, WidgetView};

use crate::theme;
use crate::viewmodel::{App, TabVm};

pub fn view(tab: &TabVm) -> impl WidgetView<App> + use<> {
    let body = tab.request().body().to_string();
    portal(
        flex_col((
            prose("Request body (sent verbatim).")
                .text_size(12.0)
                .text_color(theme::MUTED),
            FlexSpacer::Fixed(6.px()),
            text_input(body, |state: &mut App, value| {
                state.workspace.set_body(value);
            })
            .placeholder("{ \"key\": \"value\" }")
            .insert_newline(InsertNewline::OnEnter),
        ))
        .must_fill_major_axis(true)
        .padding(10.0),
    )
}
