pub mod password_details;
mod password_list;

use iced::widget::{container, row};
use iced::{Element, Left, Subscription, Task, Top};
use passepartout::PasswordInfo;

use self::{password_details::PasswordDetails, password_list::PasswordList};

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
    password_list: PasswordList,
    password_details: PasswordDetails,
}

impl Dashboard {
    pub fn new(passwords: Vec<PasswordInfo>) -> (Self, Task<Message>) {
        let (password_list, _) = PasswordList::new(passwords);
        let (password_details, _) = PasswordDetails::new();
        (
            Self {
                password_list,
                password_details,
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
                    password_list::Action::SelectEntry(entry) => {
                        // should I really create a new message here?
                        match self
                            .password_details
                            .update(password_details::Message::SelectEntry(entry))
                        {
                            password_details::Action::Run(task) => {
                                Action::Run(task.map(Message::PasswordDetails))
                            }
                            _ => Action::None,
                        }
                    }
                    _ => Action::None,
                }
            }
            Message::PasswordDetails(message) => {
                let action = self.password_details.update(message);
                match action {
                    password_details::Action::Run(task) => {
                        Action::Run(task.map(Message::PasswordDetails))
                    }
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
