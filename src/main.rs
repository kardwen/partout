use iced::advanced::graphics::image::image_rs::ImageFormat;

mod app;
mod icon;
mod macros;
mod pass;
mod screen;
mod sidebar;
mod theme;

use app::App;

pub fn main() -> iced::Result {
    let window_icon = iced::window::icon::from_file_data(
        include_bytes!("../assets/partout64.png"),
        Some(ImageFormat::Png),
    );
    let window_settings = iced::window::Settings {
        size: iced::Size {
            width: 900.0,
            height: 600.0,
        },
        icon: match window_icon {
            Ok(icon) => Some(icon),
            Err(_) => None,
        },
        ..Default::default()
    };

    iced::application(App::title, App::update, App::view)
        .subscription(App::subscription)
        .theme(App::theme)
        .font(icon::FONT_BYTES)
        .window(window_settings)
        .scale_factor(App::scale_factor)
        .run_with(App::new)
}
