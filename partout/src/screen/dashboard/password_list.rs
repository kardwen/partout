use iced::widget::{column, scrollable, text, text_input, Column};
use iced::{Center, Element, Fill, Task};

use passepartout::PasswordInfo;

#[derive(Debug, Clone)]
pub enum Message {
    SearchChanged(String),
}

pub enum Action {
    None,
    Run(Task<Message>),
    Back,
    UpdateSearch,
}

pub struct PasswordList {
    scrollbar_width: u16,
    scrollbar_margin: u16,
    scroller_width: u16,
    anchor: scrollable::Anchor,
    search: String,
    passwords: Vec<PasswordInfo>,
}

impl PasswordList {
    pub fn new(passwords: Vec<PasswordInfo>) -> (Self, Task<Message>) {
        (
            Self {
                scrollbar_width: 10,
                scrollbar_margin: 0,
                scroller_width: 10,
                anchor: scrollable::Anchor::Start,
                search: String::new(),
                passwords,
            },
            Task::none(),
        )
    }

    pub fn view(&self) -> Element<Message> {
        let search = text_input("Search...", &self.search)
            .size(20)
            .padding(10)
            .on_input(Message::SearchChanged);

        let mut rows = Column::new();
        for password in &self.passwords {
            rows = rows.push(text(password.pass_id.clone()));
        }
        let table = scrollable(rows.align_x(Center).padding([20, 0]).spacing(20))
            .direction(scrollable::Direction::Vertical(
                scrollable::Scrollbar::new()
                    .width(self.scrollbar_width)
                    .margin(self.scrollbar_margin)
                    .scroller_width(self.scroller_width)
                    .anchor(self.anchor),
            ))
            .width(Fill)
            .height(Fill);

        column![search, table].into()
    }

    pub fn update(&mut self, message: Message) -> Action {
        match message {
            Message::SearchChanged(search) => {
                self.search = search;
                Action::UpdateSearch
            }
        }
    }
}
