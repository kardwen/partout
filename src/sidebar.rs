use iced::alignment::Vertical::Center;
use iced::widget::{button, column, container, horizontal_space, row, text, vertical_space};
use iced::{Element, Subscription, Task};

use crate::icon;

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
            button(
                row!["Dashboard", horizontal_space(), icon::book()]
                    .width(105)
                    .align_y(Center),
            )
            .on_press(Message::ShowDashboard),
            button(
                row!["Settings", horizontal_space(), icon::settings()]
                    .width(105)
                    .align_y(Center),
            )
            .on_press(Message::ShowSettings),
            button("Quit").on_press(Message::Quit),
            vertical_space(),
            container(text("alpha"))
                .style(container::rounded_box)
                .padding([0, 5]),
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
