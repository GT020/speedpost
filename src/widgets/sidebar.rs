//! Sidebar: collections and request history.

use xilem::WidgetView;
use xilem::masonry::properties::types::AsUnit;
use xilem::style::Style as _;
use xilem::view::{FlexSpacer, flex_col, flex_row, prose, text_button};

use crate::icons::{self, ti};
use crate::theme;
use crate::util::truncate;
use crate::viewmodel::App;

pub fn view(state: &App) -> impl WidgetView<App> + use<> {
    let history: Vec<_> = state
        .history
        .entries()
        .iter()
        .enumerate()
        .map(|(i, entry)| {
            let label = format!("{}  {}", entry.method, truncate(&entry.url, 34));
            text_button(label, move |state: &mut App| state.select_history(i))
        })
        .collect();

    let collections: Vec<_> = state
        .collections
        .collections()
        .iter()
        .map(crate::widgets::collection_item)
        .collect();

    flex_col((
        flex_row((
            icons::icon(ti::FOLDER, 12.0, theme::MUTED),
            prose("Collections")
                .text_size(08.0)
                .text_color(theme::DARK_BG),
        ))
        .gap(4.px()),
        FlexSpacer::Fixed(6.px()),
        flex_col(collections).gap(4.px()),
        FlexSpacer::Fixed(14.px()),
        flex_row((
            icons::icon(ti::HISTORY, 12.0, theme::MUTED),
            prose("History").text_size(08.0).text_color(theme::DARK_BG),
        ))
        .gap(4.px()),
        FlexSpacer::Fixed(6.px()),
        flex_col(history).gap(4.px()),
        FlexSpacer::Flex(1.0),
    ))
    .must_fill_major_axis(true)
    .cross_axis_alignment(xilem::view::CrossAxisAlignment::Start)
    .padding(2.0)
    .background_color(theme::DARK_BG)
}
