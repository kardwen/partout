use iced::widget::{button, column, container, horizontal_space, row, text, Button, Column};
use iced::{color, Element, Fill, Left, Right, Task, Top};
use passepartout::PasswordInfo;

use crate::{icons, pass};

#[derive(Debug, Clone)]
pub enum Message {
    SelectEntry(PasswordInfo),
    ShowSecrets,
    HideSecrets,
    CopyId(PasswordInfo),
    ShowFile(PasswordInfo),
    CopyPassword(PasswordInfo),
    CopyLogin(PasswordInfo),
    FetchOtp(PasswordInfo),
    CopyOtp(PasswordInfo),
    EntryFetched(String, String),
    OtpFetched(String, String),
    IdCopied(bool),
    PasswordCopied(bool),
    LoginCopied(bool),
    OtpCopied(bool),
}

pub enum Action {
    None,
    Run(Task<Message>),
}

pub struct PasswordDetails {
    entry: Option<PasswordInfo>,
    show_secrets: bool,
    file_contents: Option<String>,
    line_count: Option<String>,
    password: Option<String>,
    login: Option<String>,
    otp: Option<String>,
}

impl PasswordDetails {
    pub fn new() -> (Self, Task<Message>) {
        (
            Self {
                entry: None,
                show_secrets: false,
                file_contents: None,
                line_count: None,
                password: None,
                login: None,
                otp: None,
            },
            Task::none(),
        )
    }

    pub fn view(&self) -> Element<Message> {
        let mut header = column![text("Details").width(Fill),];
        let mut content = Column::new().width(Fill);

        if let Some(ref entry) = self.entry {
            header = if self.show_secrets {
                header.push(
                    button(row!["Hide secrets", horizontal_space(), icons::unlock()].width(120))
                        .on_press(Message::HideSecrets),
                )
            } else {
                header.push(
                    button(row!["Show secrets", horizontal_space(), icons::lock()].width(120))
                        .on_press(Message::ShowSecrets),
                )
            };

            content = content.push(password_field(
                "Password File",
                &entry.pass_id,
                vec![button(row!["Copy", icons::clipboard()].spacing(10))
                    .on_press(Message::CopyId(entry.clone()))],
            ));
            if let Some(ref line_count) = self.line_count {
                content = content.push(password_field(
                    "Number of lines",
                    line_count,
                    vec![button(row!["Show file", icons::file()].spacing(10))
                        .on_press(Message::ShowFile(entry.clone()))],
                ));
            }
            let password = if self.show_secrets {
                self.password.as_ref().map_or("********", |p| p.as_str())
            } else {
                "********"
            };
            content = content.push(password_field(
                "Password",
                password,
                vec![button(row!["Copy", icons::clipboard()].spacing(10))
                    .on_press(Message::CopyPassword(entry.clone()))],
            ));
            if let Some(ref login) = self.login {
                content = if self.show_secrets {
                    content.push(password_field(
                        "Login",
                        login,
                        vec![button(row!["Copy", icons::clipboard()].spacing(10))
                            .on_press(Message::CopyLogin(entry.clone()))],
                    ))
                } else {
                    content.push(password_field(
                        "Login",
                        "",
                        vec![button(row!["Copy", icons::clipboard()].spacing(10))
                            .on_press(Message::CopyLogin(entry.clone()))],
                    ))
                }
            }
            if let Some(ref otp) = self.otp {
                content = if self.show_secrets {
                    content.push(password_field(
                        "One-time password (OTP)",
                        otp,
                        vec![
                            button(row!["Refresh", icons::refresh()].spacing(10))
                                .on_press(Message::FetchOtp(entry.clone())),
                            button(row!["Copy", icons::clipboard()].spacing(10))
                                .on_press(Message::CopyOtp(entry.clone())),
                        ],
                    ))
                } else {
                    content.push(password_field(
                        "One-time password (OTP)",
                        "******",
                        vec![
                            button(row!["Refresh", icons::refresh()].spacing(10))
                                .on_press(Message::FetchOtp(entry.clone())),
                            button(row!["Copy", icons::clipboard()].spacing(10))
                                .on_press(Message::CopyOtp(entry.clone())),
                        ],
                    ))
                }
            }
            column![
                header.spacing(10).padding([10, 10]),
                container(content.spacing(10))
                    .padding(10)
                    .style(container::rounded_box),
                text("Note: Copying of the file identifier and viewing the file is currently not working.")
            ]
            .into()
        } else {
            header.padding([0, 10]).into()
        }
    }

    pub fn update(&mut self, message: Message) -> Action {
        match message {
            Message::SelectEntry(entry) => {
                self.show_secrets = false;
                self.select(entry.clone());
                Action::Run(Task::perform(
                    pass::fetch_entry(entry.pass_id),
                    |(id, result)| Message::EntryFetched(id, result),
                ))
            }
            Message::EntryFetched(id, result) => self.update_fields(id, result),
            Message::ShowSecrets => {
                self.show_secrets();
                // Refresh OTP
                if let Some(entry) = &self.entry {
                    Action::Run(Task::perform(
                        pass::fetch_otp(entry.pass_id.clone()),
                        |(id, result)| Message::OtpFetched(id, result),
                    ))
                } else {
                    Action::None
                }
            }
            Message::HideSecrets => {
                self.hide_secrets();
                Action::None
            }
            Message::CopyId(entry) => {
                Action::Run(Task::perform(pass::copy_id(entry.pass_id), |result| {
                    Message::IdCopied(result)
                }))
            }
            Message::IdCopied(_) => Action::None,
            Message::ShowFile(_) => Action::None, // TODO: implement
            Message::CopyPassword(entry) => Action::Run(Task::perform(
                pass::copy_password(entry.pass_id),
                Message::PasswordCopied,
            )),
            Message::PasswordCopied(_) => Action::None,
            Message::CopyLogin(entry) => {
                Action::Run(Task::perform(pass::copy_login(entry.pass_id), |result| {
                    Message::LoginCopied(result)
                }))
            }
            Message::LoginCopied(_) => Action::None,
            Message::CopyOtp(entry) => {
                Action::Run(Task::perform(pass::copy_otp(entry.pass_id), |result| {
                    Message::OtpCopied(result)
                }))
            }
            Message::OtpCopied(_) => Action::None,
            Message::FetchOtp(entry) => Action::Run(Task::perform(
                pass::fetch_otp(entry.pass_id),
                |(id, result)| Message::OtpFetched(id, result),
            )),
            Message::OtpFetched(id, otp) => {
                if let Some(ref entry) = self.entry {
                    if entry.pass_id == id {
                        self.otp = Some(otp);
                    }
                }
                Action::None
            }
        }
    }

    fn select(&mut self, entry: PasswordInfo) {
        self.entry = Some(entry);
        self.file_contents = None;
        self.line_count = None;
        self.password = None;
        self.login = None;
        self.otp = None;
    }

    fn show_secrets(&mut self) {
        self.show_secrets = true;
    }

    fn hide_secrets(&mut self) {
        self.show_secrets = false;
    }

    fn update_fields(&mut self, id: String, file_contents: String) -> Action {
        if let Some(ref entry) = self.entry {
            let pass_id = entry.pass_id.clone();
            if id != pass_id {
                return Action::None;
            }
            self.file_contents = Some(file_contents.clone());

            let mut lines = file_contents.lines();
            let mut count = 0;
            if let Some(password) = lines.next() {
                self.password = Some(password.to_string());
                count += 1;
            }
            if let Some(login) = lines.next() {
                self.login = Some(login.to_string());
                count += 1;
            }

            let mut next_line = lines.next();
            let mut has_otp = false;
            while let Some(line) = next_line {
                // One-time password (OTP)
                if line.starts_with("otpauth://") {
                    has_otp = true;
                }
                count += 1;
                next_line = lines.next();
            }
            self.line_count = Some(count.to_string());

            if has_otp {
                self.otp = Some("*".repeat(6));
                return Action::Run(Task::perform(
                    pass::fetch_otp(entry.pass_id.clone()),
                    |(id, result)| Message::OtpFetched(id, result),
                ));
            }
        }
        Action::None
    }
}

fn password_field<'a>(
    label: &'a str,
    value: &'a str,
    buttons: Vec<Button<'a, Message>>,
) -> Element<'a, Message> {
    row![
        column![text(label).color(color!(0x0055ff)), text(value)].align_x(Left),
        horizontal_space(),
        Column::with_children(buttons.into_iter().map(|button| { button.into() }))
            .spacing(10)
            .align_x(Right),
    ]
    .align_y(Top)
    .into()
}
