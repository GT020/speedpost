//! Tabler icon support.
//!
//! Icons are the outline set from <https://github.com/tabler/tabler-icons>
//! (MIT licensed). The webfont in `assets/fonts/tabler-icons.ttf` is embedded in
//! the binary and registered with the window in `main.rs`; each icon is then drawn
//! as a single glyph of that font.
//!
//! Render one with [`icon`], e.g. `icons::icon(ti::SEND, 16.0, theme::ORANGE)`,
//! passing any of the named constants from [`ti`] (or a raw Tabler icon name).
//!
//! Any of the 5000+ Tabler icon names can be used through [`icon`]; the [`ti`]
//! module just offers named constants for the ones this app relies on.

mod table;

use xilem::WidgetView;
use xilem::masonry::parley::style::FontFamily;
use xilem::masonry::peniko::Color;
use xilem::style::Style as _;
use xilem::view::label;

pub use table::TABLER_GLYPHS;

/// Font family name declared by the bundled Tabler icons font.
pub const FONT_FAMILY: &str = "tabler-icons";

/// The bundled font, embedded so the application ships without external assets.
pub const FONT: &[u8] = include_bytes!("../assets/fonts/tabler-icons.ttf");

/// Look up the display glyph for a Tabler icon by name (e.g. `"send"`).
pub fn glyph(name: &str) -> Option<char> {
    TABLER_GLYPHS
        .binary_search_by(|(n, _)| (*n).cmp(name))
        .ok()
        .map(|i| TABLER_GLYPHS[i].1)
}

/// A single, non-interactive Tabler icon rendered in the given `color` at `size`.
///
/// Unknown names fall back to the replacement character so a typo degrades to a
/// visible box rather than silently shifting the surrounding layout.
pub fn icon<State: 'static, Action: 'static>(
    name: &str,
    size: f32,
    color: Color,
) -> impl WidgetView<State, Action> + use<State, Action> {
    let text = glyph(name).unwrap_or('\u{FFFD}').to_string();
    label(text)
        .text_size(size)
        .font(FontFamily::Named(FONT_FAMILY.into()))
        .color(color)
}

/// Named constants for the Tabler icons used across the app.
///
/// Keeping them here avoids typos in string literals and makes it easy to swap an
/// icon. The name matches the Tabler icon name (without the `ti-` prefix).
pub mod ti {
    #![allow(dead_code)]

    pub const SEND: &str = "send";
    pub const HISTORY: &str = "history";
    pub const FOLDER: &str = "folder";
    pub const FILE: &str = "file";
    pub const WORLD: &str = "world";
    pub const SETTINGS: &str = "settings";
    pub const TRASH: &str = "trash";
    pub const PLUS: &str = "plus";
    pub const SEARCH: &str = "search";
    pub const X: &str = "x";
    pub const CODE: &str = "code";
    pub const CLOCK: &str = "clock";
    pub const CHECK: &str = "circle-check";
    pub const ALERT: &str = "alert-triangle";
    pub const BOLT: &str = "bolt";
    pub const LOADER: &str = "loader-2";
    pub const CHEVRON_DOWN: &str = "chevron-down";
    pub const CHEVRON_RIGHT: &str = "chevron-right";
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn table_is_sorted_for_binary_search() {
        assert!(TABLER_GLYPHS.windows(2).all(|w| w[0].0 < w[1].0));
    }

    #[test]
    fn known_icons_resolve() {
        for name in ["send", "history", "folder", "x", "plus"] {
            assert!(glyph(name).is_some(), "missing glyph for {name}");
        }
    }

    #[test]
    fn unknown_icons_are_none() {
        assert_eq!(glyph("definitely-not-a-tabler-icon"), None);
    }
}
