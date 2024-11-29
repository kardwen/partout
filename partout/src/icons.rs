use iced::widget::{text, Text};
use iced::Font;
use iced_fonts::nerd::{icon_to_string, Nerd};
use iced_fonts::{NERD_FONT, NERD_FONT_BYTES};

pub const FONT: Font = NERD_FONT;
pub const FONT_BYTES: &[u8] = NERD_FONT_BYTES;

pub fn clipboard() -> Text<'static> {
    text(icon_to_string(Nerd::Clipboard)).font(FONT)
}

pub fn refresh() -> Text<'static> {
    text(icon_to_string(Nerd::TimerRefresh)).font(FONT)
}

pub fn lock() -> Text<'static> {
    text(icon_to_string(Nerd::Lock)).font(FONT)
}

pub fn unlock() -> Text<'static> {
    text(icon_to_string(Nerd::Unlock)).font(FONT)
}

pub fn file() -> Text<'static> {
    text(icon_to_string(Nerd::FileCabinet)).font(FONT)
}

pub fn view() -> Text<'static> {
    text(icon_to_string(Nerd::GistSecret)).font(FONT)
}
