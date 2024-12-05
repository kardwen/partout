use iced::{
    widget::{text, text::LineHeight, Text},
    Font,
};
use std::sync::OnceLock;
use verglas::{build_icon_map, IconMap};

use crate::{define_icons, theme};

pub const FONT: Font = iced::Font::with_name("partout-icons");
pub const FONT_BYTES: &[u8] = include_bytes!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/assets/partout-icons.ttf"
));

static ICON_MAP: OnceLock<IconMap> = OnceLock::new();

/// Returns an [`IconMap`] that is only build once when first called
fn get_icon_map() -> &'static IconMap {
    ICON_MAP.get_or_init(|| {
        build_icon_map(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/assets/partout-icons.ttf"
        ))
        .unwrap_or_else(|e| {
            eprintln!("icon map creation failed: {:?}", e);
            IconMap::new()
        })
    })
}

define_icons! {
    alert => "jam/alert",
    book => "jam/book",
    brush => "jam/brush",
    chronometer => "jam/chronometer",
    clipboard => "jam/clipboard",
    document => "jam/document",
    hidden => "jam/eye-close",
    visible => "jam/eye",
    file => "jam/file",
    info => "jam/info",
    key => "jam/key",
    login => "jam/log-in",
    refresh => "jam/refresh",
    search => "jam/search",
    settings => "jam/settings-alt"
}

fn icon<'a>(name: &str) -> Text<'a> {
    let unicode = get_icon_map().get(name).copied().unwrap_or_else(|| {
        eprintln!("icon '{}' not found", name);
        char::REPLACEMENT_CHARACTER
    });

    text(unicode.to_string())
        .line_height(LineHeight::Relative(1.0))
        .size(theme::ICON_SIZE)
        .font(FONT)
}
