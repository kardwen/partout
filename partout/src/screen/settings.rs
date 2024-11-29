use iced::widget::{column, horizontal_rule, pick_list, text};
use iced::{Element, Fill, Subscription, Task, Theme};

#[derive(Debug, Clone)]
pub enum Message {
    ThemeChanged(Theme),
}

pub enum Action {
    None,
    ChangeTheme(Theme),
    Run(Task<Message>),
    Back,
}

pub struct Settings {
    theme: Theme,
}

impl Settings {
    pub fn new(theme: &mut Theme) -> (Self, Task<Message>) {
        // TODO: theme should not be cloned
        (
            Self {
                theme: theme.clone(),
            },
            Task::none(),
        )
    }

    pub fn title(&self) -> String {
        "Partout - Settings".to_owned()
    }

    pub fn update(&mut self, message: Message) -> Action {
        match message {
            Message::ThemeChanged(theme) => {
                self.theme = theme.clone();
                Action::ChangeTheme(theme)
            }
        }
    }

    pub fn view(&self) -> Element<Message> {
        let choose_theme = column![
            text("Theme:"),
            pick_list(Theme::ALL, Some(&self.theme), Message::ThemeChanged).width(Fill),
        ]
        .spacing(10);

        let content = column![
            text("Settings"),
            horizontal_rule(38),
            choose_theme,
            horizontal_rule(38),
            text("Note: Settings cannot be saved at the moment."),
        ]
        .spacing(20)
        .padding(20);

        content.into()
    }

    pub fn subscription(&self) -> Subscription<Message> {
        Subscription::none()
    }
}
