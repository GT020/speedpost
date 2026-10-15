//! Response viewer tab selector (Body / Headers).

use xilem::WidgetView;
use xilem::masonry::properties::types::AsUnit;
use xilem::style::Style as _;
use xilem::view::{flex_row, text_button};

use crate::model::RespTab;
use crate::viewmodel::{App, TabVm};

pub fn view(tab: &TabVm) -> impl WidgetView<App> + use<> {
    let current = tab.response().tab();
    flex_row((
        text_button("Body", move |state: &mut App| {
            state.workspace.set_response_tab(RespTab::Body)
        })
        .disabled(current == RespTab::Body),
        text_button("Headers", move |state: &mut App| {
            state.workspace.set_response_tab(RespTab::Headers)
        })
        .disabled(current == RespTab::Headers),
    ))
    .gap(4.px())
    .padding(8.0)
}
