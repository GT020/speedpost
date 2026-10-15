//! Response body / headers viewer.

use xilem::WidgetView;
use xilem::style::Style as _;
use xilem::view::{portal, prose, sized_box};

use crate::model::RespTab;
use crate::theme;
use crate::viewmodel::{App, TabVm};

pub fn view(tab: &TabVm) -> impl WidgetView<App> + use<> {
    let text = match (tab.response().response(), tab.response().tab()) {
        (Some(response), RespTab::Body) => response.body.clone(),
        (Some(response), RespTab::Headers) => response
            .headers
            .iter()
            .map(|(key, value)| format!("{key}: {value}"))
            .collect::<Vec<_>>()
            .join("\n"),
        (None, _) => String::new(),
    };

    portal(
        sized_box(prose(text).text_size(13.0))
            .padding(10.0)
            .background_color(theme::DARK_BG),
    )
}
