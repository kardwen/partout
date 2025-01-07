use iced::widget::{container, row};
use iced::window;
use iced::{Element, Fill, Left, Subscription, Task, Theme, Top};
use passepartout::PasswordStore;

use crate::{
    screen::{
        self,
        dashboard::{self, Dashboard},
        settings::{self, Settings},
        Screen,
    },
    sidebar::{self, Sidebar},
};

#[derive(Debug, Clone)]
pub enum Message {
    Loading,
    Sidebar(sidebar::Message),
    Dashboard(dashboard::Message),
    Settings(settings::Message),
}

pub struct App {
    sidebar: Sidebar,
    screen: Screen,
    theme: Theme,
    scale_factor: f64,
    store: PasswordStore,
}

impl App {
    pub fn new() -> (Self, Task<Message>) {
        let store = PasswordStore::new();
        let (sidebar, _) = Sidebar::new();
        let screen = Screen::Loading;
        (
            Self {
                sidebar,
                screen,
                theme: Theme::default(),
                scale_factor: 0.75,
                store,
            },
            Task::none(),
        )
    }

    pub fn title(&self) -> String {
        match &self.screen {
            Screen::Loading => "Partout".to_owned(),
            Screen::Dashboard(dashboard) => dashboard.title(),
            Screen::Settings(settings) => settings.title(),
        }
    }

    pub fn scale_factor(&self) -> f64 {
        self.scale_factor
    }

    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::Loading => Task::none(),
            Message::Sidebar(message) => {
                let action = self.sidebar.update(message);
                match action {
                    sidebar::Action::ShowDashboard => {
                        let (dashboard, _) = Dashboard::new(self.store.passwords.clone());
                        self.screen = Screen::Dashboard(dashboard);
                    }
                    sidebar::Action::ShowSettings => {
                        let (settings, _) = Settings::new(&mut self.theme);
                        self.screen = Screen::Settings(settings);
                    }
                    sidebar::Action::Quit => return window::get_latest().and_then(window::close),
                    sidebar::Action::None => (),
                }
                Task::none()
            }
            Message::Dashboard(message) => {
                if let Screen::Dashboard(dashboard) = &mut self.screen {
                    let action = dashboard.update(message);
                    return match action {
                        dashboard::Action::Run(task) => task.map(Message::Dashboard),
                        _ => Task::none(),
                    };
                }
                Task::none()
            }
            Message::Settings(message) => {
                if let Screen::Settings(settings) = &mut self.screen {
                    let action = settings.update(message);
                    return match action {
                        settings::Action::ChangeTheme(theme) => {
                            self.theme = theme;
                            Task::none()
                        }
                        _ => Task::none(),
                    };
                }
                Task::none()
            }
        }
    }

    pub fn view(&self) -> Element<Message> {
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

    pub fn subscription(&self) -> Subscription<Message> {
        Subscription::none()
    }

    pub fn theme(&self) -> Theme {
        self.theme.clone()
    }
}
