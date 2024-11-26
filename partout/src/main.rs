mod screen;
mod sidebar;

use iced::widget::{container, row};
use iced::{color, window};
use iced::{Element, Fill, Left, Subscription, Task, Theme, Top};

use screen::{
    dashboard::{self, Dashboard},
    settings::{self, Settings},
    Screen,
};
use sidebar::Sidebar;

pub fn main() -> iced::Result {
    iced::application(App::title, App::update, App::view)
        .subscription(App::subscription)
        .theme(App::theme)
        .window_size((600.0, 500.0))
        .run_with(App::new)
}

#[derive(Debug, Clone)]
enum Message {
    Loading,
    Sidebar(sidebar::Message),
    Dashboard(dashboard::Message),
    Settings(settings::Message),
}

struct App {
    sidebar: Sidebar,
    screen: Screen,
    theme: Theme,
}

impl App {
    pub fn new() -> (Self, Task<Message>) {
        let (sidebar, _) = Sidebar::new();
        let screen = Screen::Loading;
        (
            Self {
                sidebar,
                screen,
                theme: Theme::TokyoNight,
            },
            Task::none(),
        )
    }

    fn title(&self) -> String {
        match &self.screen {
            Screen::Loading => "Partout".to_owned(),
            Screen::Dashboard(dashboard) => dashboard.title(),
            Screen::Settings(settings) => settings.title(),
        }
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::Loading => {}
            Message::Sidebar(message) => {
                let action = self.sidebar.update(message);
                match action {
                    sidebar::Action::ShowDashboard => {
                        let (dashboard, _) = Dashboard::new();
                        self.screen = Screen::Dashboard(dashboard);
                    }
                    sidebar::Action::ShowSettings => {
                        let (settings, _) = Settings::new(&mut self.theme);
                        self.screen = Screen::Settings(settings);
                    }
                    sidebar::Action::Quit => return window::get_latest().and_then(window::close),
                    sidebar::Action::None => (),
                }
            }
            Message::Dashboard(message) => {
                if let Screen::Dashboard(dashboard) = &mut self.screen {
                    let action = dashboard.update(message);
                    match action {
                        _ => {}
                    }
                }
            }
            Message::Settings(message) => {
                if let Screen::Settings(settings) = &mut self.screen {
                    let action = settings.update(message);
                    match action {
                        settings::Action::ChangeTheme(theme) => self.theme = theme,
                        _ => {}
                    }
                }
            }
        }
        Task::none()
    }

    fn view(&self) -> Element<Message> {
        let sidebar = container(
            self.sidebar
                .view()
                // .explain(color!(0x0000ff))
                .map(Message::Sidebar),
        )
        .align_x(Left)
        .align_y(Top);

        let screen = container(match &self.screen {
            Screen::Loading => screen::loading(),
            Screen::Dashboard(dashboard) => dashboard.view().map(Message::Dashboard),
            Screen::Settings(settings) => settings.view().map(Message::Settings),
        })
        .width(Fill);

        container(row![sidebar, screen]).padding(5).into()
    }

    fn subscription(&self) -> Subscription<Message> {
        Subscription::none()
    }

    fn theme(&self) -> Theme {
        self.theme.clone()
    }
}
