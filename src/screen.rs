use iced::{widget::horizontal_space, Element};

pub mod dashboard;
pub mod settings;

use self::dashboard::Dashboard;
use self::settings::Settings;

pub enum Screen {
    Loading,
    Settings(Settings),
    Dashboard(Dashboard),
}

pub fn loading<'a, Message: 'a>() -> Element<'a, Message> {
    horizontal_space().into()
}
