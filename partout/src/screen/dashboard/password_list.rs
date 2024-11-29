use iced::widget::{
    button, column, container, horizontal_space, hover, row, scrollable, text, text_input, Column,
};
use iced::{Element, Fill, Font, Left, Right, Task};

use crate::icons;
use passepartout::PasswordInfo;

#[derive(Debug, Clone)]
pub enum Message {
    SearchChanged(String),
    SelectEntry(PasswordInfo),
}

pub enum Action {
    None,
    Run(Task<Message>),
    Back,
    UpdateSearch,
    SelectEntry(PasswordInfo),
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
        let filter = !self.search.is_empty();
        for entry in &self.passwords {
            if filter
                && !entry
                    .pass_id
                    .to_lowercase()
                    .contains(&self.search.to_lowercase())
            {
                continue;
            }
            rows = rows.push(password_card(entry));
        }
        let list = scrollable(row![
            rows.align_x(Left).spacing(10),
            horizontal_space().width(12)
        ])
        .direction(scrollable::Direction::Vertical(
            scrollable::Scrollbar::new()
                .width(self.scrollbar_width)
                .margin(self.scrollbar_margin)
                .scroller_width(self.scroller_width)
                .anchor(self.anchor),
        ))
        .width(Fill)
        .height(Fill);

        column![search, list].into()
    }

    pub fn update(&mut self, message: Message) -> Action {
        match message {
            Message::SearchChanged(search) => {
                self.search = search;
                Action::UpdateSearch
            }
            Message::SelectEntry(entry) => Action::SelectEntry(entry),
        }
    }
}

fn password_card(entry: &PasswordInfo) -> Element<Message> {
    let title = {
        const LIMIT: usize = 40;

        let name = entry.pass_id.clone();

        if name.len() < LIMIT {
            text(name)
        } else {
            text!("{}...", &name[0..LIMIT])
        }
        .font(Font::MONOSPACE)
    };

    let details = container(
        button(row!["View", icons::view()].spacing(10))
            .on_press(Message::SelectEntry(entry.clone())),
    )
    .width(Fill)
    .padding(10)
    .align_x(Right)
    .center_y(Fill);

    let card = container(title)
        .width(Fill)
        .padding(15)
        .style(container::rounded_box);

    hover(card, details)
}
