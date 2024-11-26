use iced::widget::{button, column, vertical_space};
use iced::{Element, Subscription, Task};

#[derive(Debug, Clone)]
pub enum Message {
    ShowSettings,
    ShowDashboard,
    Quit,
}

pub enum Action {
    None,
    ShowSettings,
    ShowDashboard,
    Quit,
}

pub struct Sidebar {}

impl Sidebar {
    pub fn new() -> (Self, Task<Message>) {
        (Self {}, Task::none())
    }

    pub fn update(&mut self, message: Message) -> Action {
        match message {
            Message::ShowDashboard => Action::ShowDashboard,
            Message::ShowSettings => Action::ShowSettings,
            Message::Quit => Action::Quit,
        }
    }

    pub fn view(&self) -> Element<Message> {
        column![
            button("Dashboard").on_press(Message::ShowDashboard),
            button("Settings").on_press(Message::ShowSettings),
            button("Quit").on_press(Message::Quit),
            vertical_space()
        ]
        .spacing(10)
        .padding(10)
        .max_width(250)
        .into()
    }

    pub fn subscription(&self) -> Subscription<Message> {
        Subscription::none()
    }
}
