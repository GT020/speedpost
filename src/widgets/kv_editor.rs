//! Editable key/value table, reused for both query params and headers.

use xilem::WidgetView;
use xilem::masonry::properties::types::AsUnit;
use xilem::style::Style as _;
use xilem::view::{
    FlexExt as _, FlexSpacer, flex_col, flex_row, portal, prose, text_button, text_input,
};

use crate::model::KVKind;
use crate::theme;
use crate::viewmodel::{App, TabVm};

pub fn view(tab: &TabVm, kind: KVKind) -> impl WidgetView<App> + use<> {
    let rows: Vec<_> = tab
        .request()
        .kv(kind)
        .iter()
        .cloned()
        .enumerate()
        .map(|(i, kv)| {
            let key = text_input(kv.key, move |state: &mut App, value| {
                state.workspace.set_kv_key(kind, i, value);
            })
            .placeholder("Key")
            .flex(1.0);
            let value = text_input(kv.value, move |state: &mut App, value| {
                state.workspace.set_kv_value(kind, i, value);
            })
            .placeholder("Value")
            .flex(1.0);
            let remove = text_button("✕", move |state: &mut App| {
                state.workspace.remove_kv(kind, i);
            });
            flex_row((key, value, remove)).gap(6.px())
        })
        .collect();

    let add = text_button("+ Add row", move |state: &mut App| {
        state.workspace.add_kv(kind);
    });

    portal(
        flex_col((
            prose("Key / Value rows are merged into the request.")
                .text_size(12.0)
                .text_color(theme::MUTED),
            FlexSpacer::Fixed(6.px()),
            flex_col(rows).gap(6.px()),
            FlexSpacer::Fixed(8.px()),
            add,
        ))
        .padding(10.0),
    )
}
