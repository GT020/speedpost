//! Status line summarizing the last response.

use xilem::WidgetView;
use xilem::masonry::properties::types::AsUnit;
use xilem::style::Style as _;
use xilem::view::{flex_row, prose, sized_box};

use crate::icons::{self, ti};
use crate::theme;
use crate::util::human_size;
use crate::viewmodel::{App, TabVm};

pub fn view(tab: &TabVm) -> impl WidgetView<App> + use<> {
    let (text, color, icon) = if tab.request().sending() {
        ("Sending request…".to_string(), theme::AMBER, ti::LOADER)
    } else if let Some(response) = tab.response().response() {
        if let Some(err) = &response.error {
            (format!("Error: {err}"), theme::RED, ti::ALERT)
        } else {
            let color = match response.status {
                200..=299 => theme::GREEN,
                300..=399 => theme::AMBER,
                _ => theme::RED,
            };
            (
                format!(
                    "{} {}  •  {} ms  •  {}",
                    response.status,
                    response.status_text,
                    response.duration_ms,
                    human_size(response.size),
                ),
                color,
                if response.status < 400 {
                    ti::CHECK
                } else {
                    ti::ALERT
                },
            )
        }
    } else {
        (
            "No response yet — hit Send.".to_string(),
            theme::MUTED,
            ti::CLOCK,
        )
    };

    sized_box(
        flex_row((
            icons::icon(icon, 14.0, color),
            prose(text).text_size(14.0).text_color(color),
        ))
        .gap(6.px()),
    )
    .expand_width()
    .padding(8.0)
    .background_color(theme::PANEL_BG)
}
