use iced::widget::{container, text};
use iced::{Element, Fill, Task};

#[derive(Debug, Clone)]
pub enum Message {}

pub enum Action {
    None,
    Run(Task<Message>),
    Back,
}

pub struct PasswordDetails {}

impl PasswordDetails {
    pub fn new() -> (Self, Task<Message>) {
        (Self {}, Task::none())
    }

    pub fn view(&self) -> Element<Message> {
        container(text("Details").width(Fill)).into()
    }

    pub fn update(&mut self, message: Message) -> Action {
        let _ = message;
        Action::None
    }
}
