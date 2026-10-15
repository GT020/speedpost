use xilem::masonry::properties::types::AsUnit;
use xilem::style::Style as _;
use xilem::view::{button, flex_row, label};
use xilem::{Color, WidgetView};

use super::hover::hover_card;
use crate::theme;
use crate::util::truncate;
use crate::viewmodel::App;
use crate::viewmodel::collections::RequestItemVm;

/// Accent color used to render an HTTP method, following the usual REST conventions.
fn method_color(method: &str) -> Color {
    match method.to_ascii_uppercase().as_str() {
        "GET" => theme::GREEN,
        "POST" => theme::AMBER,
        "PUT" => theme::ORANGE,
        "PATCH" => theme::AMBER,
        "DELETE" => theme::RED,
        _ => theme::MUTED,
    }
}

/// Renders a single request leaf node.
/// Indentation and guide lines are handled consistently by the parent expander.
pub fn view(req: &RequestItemVm) -> impl WidgetView<App> + use<> {
    let req_id = req.id();

    let method = label(req.method().to_ascii_uppercase())
        .text_size(11.0)
        .color(method_color(req.method()));
    let name = label(truncate(req.name(), 22))
        .text_size(11.0)
        .color(if req.is_selected {
            theme::ORANGE
        } else {
            theme::MUTED
        });

    let button_view = button(flex_row((method, name)).gap(6.px()), move |st: &mut App| {
        st.open_request(req_id);
    })
    .background_color(Color::TRANSPARENT)
    .active_background_color(Color::TRANSPARENT)
    .disabled_background_color(Color::TRANSPARENT)
    .border_color(Color::TRANSPARENT)
    .hovered_border_color(Color::TRANSPARENT)
    .border_width(0.0)
    .padding(1.0);

    hover_card(button_view, Color::from_rgba8(0, 0, 0, 24))
}
