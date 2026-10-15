//! Application title bar.

use xilem::masonry::properties::types::AsUnit;
use xilem::style::Style as _;
use xilem::view::{FlexSpacer, flex_row, prose, sized_box};
use xilem::{Color, WidgetView};

use crate::theme;
use crate::viewmodel::App;

pub fn view() -> impl WidgetView<App> + use<> {
    sized_box(flex_row((
        crate::icons::icon(crate::icons::ti::BOLT, 18.0, Color::WHITE),
        FlexSpacer::Fixed(6.px()),
        prose("Speedpost").text_size(18.0).text_color(Color::WHITE),
        FlexSpacer::Fixed(8.px()),
        prose("a Postman-style HTTP client")
            .text_size(13.0)
            .text_color(Color::from_rgb8(0xFF, 0xE0, 0xD0)),
        FlexSpacer::Flex(1.0),
    )))
    .expand_width()
    .padding(10.0)
    .background_color(theme::ORANGE)
}
