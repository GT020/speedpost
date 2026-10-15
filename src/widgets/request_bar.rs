//! Method / URL input row with the Send button.

use xilem::WidgetView;
use xilem::masonry::properties::types::AsUnit;
use xilem::style::Style as _;
use xilem::view::{FlexExt as _, FlexSpacer, flex_row, sized_box, text_button, text_input};

use crate::theme;
use crate::viewmodel::{App, TabVm};

pub fn view(tab: &TabVm) -> impl WidgetView<App> + use<> {
    let method = tab.request().method().to_string();
    let url = tab.request().url().to_string();
    let sending = tab.request().sending();

    let method_input = sized_box(
        text_input(method, |state: &mut App, value| {
            state.workspace.set_method(value);
        })
        .placeholder("GET"),
    )
    .width(90.px());

    let url_input = text_input(url, |state: &mut App, value| {
        state.workspace.set_url(value);
    })
    .placeholder("https://api.example.com/endpoint")
    .on_enter(|state: &mut App, _| state.send())
    .flex(1.0);

    let send = text_button(
        if sending { "Sending…" } else { "Send" },
        |state: &mut App| state.send(),
    )
    .disabled(sending)
    .background_color(if sending { theme::MUTED } else { theme::ORANGE });

    flex_row((
        method_input,
        FlexSpacer::Fixed(2.px()),
        url_input,
        FlexSpacer::Fixed(2.px()),
        send,
    ))
    .gap(6.px())
    .padding(8.0)
}
