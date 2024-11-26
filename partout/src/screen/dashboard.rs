mod password_details;
mod password_list;

use iced::widget::{container, row};
use iced::{Element, Left, Subscription, Task, Top};
use std::sync::mpsc;

use self::password_details::PasswordDetails;
use self::password_list::PasswordList;
use passepartout::PasswordStore;

#[derive(Debug, Clone)]
pub enum Message {
    PasswordList(password_list::Message),
    PasswordDetails(password_details::Message),
}

pub enum Action {
    None,
    Run(Task<Message>),
    Back,
}

pub struct Dashboard {
    password_store: PasswordStore,
    password_list: PasswordList,
    password_details: PasswordDetails,
}

impl Dashboard {
    pub fn new() -> (Self, Task<Message>) {
        let (event_tx, _event_rx) = mpsc::channel();
        let password_store = PasswordStore::new(event_tx);
        let (password_list, _) = PasswordList::new(password_store.passwords.clone());
        let (password_details, _) = PasswordDetails::new();
        (
            Self {
                password_list,
                password_details,
                password_store,
            },
            Task::none(),
        )
    }

    pub fn title(&self) -> String {
        "Partout".to_owned()
    }

    pub fn update(&mut self, message: Message) -> Action {
        match message {
            Message::PasswordList(message) => {
                let action = self.password_list.update(message);
                match action {
                    _ => Action::None,
                }
            }
            Message::PasswordDetails(message) => {
                let action = self.password_details.update(message);
                match action {
                    _ => Action::None,
                }
            }
        }
    }

    pub fn view(&self) -> Element<Message> {
        let password_list = container(
            self.password_list
                .view()
                // .explain(color!(0x0000ff))
                .map(Message::PasswordList),
        )
        .align_x(Left)
        .align_y(Top);

        let password_details = container(
            self.password_details
                .view()
                // .explain(color!(0x0000ff))
                .map(Message::PasswordDetails),
        )
        .align_x(Left)
        .align_y(Top);

        row![password_list, password_details].spacing(5).into()
    }

    pub fn subscription(&self) -> Subscription<Message> {
        Subscription::none()
    }
}
