//! Request editor tab selector (Params / Headers / Body).

use xilem::WidgetView;
use xilem::masonry::properties::types::AsUnit;
use xilem::style::Style as _;
use xilem::view::{flex_row, text_button};

use crate::model::ReqTab;
use crate::viewmodel::{App, TabVm};

pub fn view(tab: &TabVm) -> impl WidgetView<App> + use<> {
    let current = tab.request().tab();
    flex_row((
        text_button("Params", move |state: &mut App| {
            state.workspace.set_tab(ReqTab::Params)
        })
        .disabled(current == ReqTab::Params),
        text_button("Headers", move |state: &mut App| {
            state.workspace.set_tab(ReqTab::Headers)
        })
        .disabled(current == ReqTab::Headers),
        text_button("Body", move |state: &mut App| {
            state.workspace.set_tab(ReqTab::Body)
        })
        .disabled(current == ReqTab::Body),
    ))
    .gap(4.px())
    .padding(8.0)
}
