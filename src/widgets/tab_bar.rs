//! Workspace tab bar: one entry per open document, plus a new-tab button.

use xilem::masonry::properties::types::AsUnit;
use xilem::style::Style as _;
use xilem::view::{button, flex_row, text_button};
use xilem::{AnyWidgetView, WidgetView};

use crate::icons::{self, ti};
use crate::theme;
use crate::viewmodel::App;

pub fn view(state: &App) -> impl WidgetView<App> + use<> {
    let active = state.workspace.active_id();
    let mut children: Vec<Box<AnyWidgetView<App>>> = Vec::new();

    for tab in state.workspace.tabs() {
        let id = tab.id();
        let is_active = active == Some(id);
        let background = if is_active {
            theme::PANEL_BG
        } else {
            theme::DARK_BG
        };
        let title = tab.title().to_string();

        let select = text_button(title, move |st: &mut App| st.workspace.activate(id))
            .background_color(background);
        let close = button(
            icons::icon(ti::X, 12.0, theme::MUTED),
            move |st: &mut App| st.workspace.close(id),
        );

        children.push(flex_row((select, close)).gap(2.px()).boxed());
    }

    children.push(
        button(icons::icon(ti::PLUS, 13.0, theme::MUTED), |st: &mut App| {
            st.workspace.open_scratch();
        })
        .boxed(),
    );

    flex_row(children)
        .gap(4.px())
        .padding(4.0)
        .background_color(theme::DARK_BG)
}
