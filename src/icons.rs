use iced::widget::{text, text::LineHeight, Text};
use iced::Font;

use crate::theme;

pub const FONT_BYTES: &[u8] = include_bytes!("../assets/fonts/partout-icons.ttf");
pub const FONT: Font = iced::Font::with_name("partout-icons");

pub fn clipboard() -> Text<'static> {
    // clipboard
    icon('\u{F101}')
}

pub fn document() -> Text<'static> {
    // document
    icon('\u{F102}')
}

pub fn key() -> Text<'static> {
    // key
    icon('\u{F103}')
}

pub fn refresh() -> Text<'static> {
    // refresh
    icon('\u{F104}')
}

pub fn brush() -> Text<'static> {
    // brush
    icon('\u{F105}')
}

pub fn hidden() -> Text<'static> {
    // eye-closed
    icon('\u{F106}')
}

pub fn info() -> Text<'static> {
    // info
    icon('\u{F107}')
}

pub fn file() -> Text<'static> {
    // file
    icon('\u{F108}')
}

pub fn book() -> Text<'static> {
    // book
    icon('\u{F109}')
}

pub fn chronometer() -> Text<'static> {
    // chronometer
    icon('\u{F10A}')
}

pub fn alert() -> Text<'static> {
    // alert
    icon('\u{F10B}')
}

pub fn search() -> Text<'static> {
    //search
    icon('\u{F10C}')
}

pub fn settings() -> Text<'static> {
    // settings-alt
    icon('\u{F10D}')
}

pub fn login() -> Text<'static> {
    // log-in
    icon('\u{F10E}')
}

pub fn visible() -> Text<'static> {
    // eye
    icon('\u{F10F}')
}

fn icon<'a>(unicode: char) -> Text<'a> {
    text(unicode.to_string())
        .line_height(LineHeight::Relative(1.0))
        .size(theme::ICON_SIZE)
        .font(FONT)
}
