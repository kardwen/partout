use iced::alignment::Vertical::Center;
use iced::widget::{button, column, container, horizontal_space, row, text, Button, Column};
use iced::{color, Element, Fill, Left, Right, Task, Top};
use std::path::PathBuf;

use passepartout::{PasswordInfo, PasswordStore};

use crate::icon;

#[derive(Debug, Clone)]
pub enum Message {
    SelectEntry(PasswordInfo),
    ShowSecrets,
    HideSecrets,
    CopyId(PasswordInfo),
    ShowFile,
    CopyPassword(PasswordInfo),
    CopyLogin(PasswordInfo),
    FetchOtp(PasswordInfo),
    CopyOtp(PasswordInfo),
    EntryFetched(String, String),
    OtpFetched(String, String),
    // TODO: Use proper results
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
    show_file: bool,
    status: Option<String>,
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
                show_file: false,
                status: None,
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
                    button(
                        row!["Hide secrets", horizontal_space(), icon::visible()]
                            .width(125)
                            .align_y(Center),
                    )
                    .on_press(Message::HideSecrets),
                )
            } else {
                header.push(
                    button(
                        row!["Show secrets", horizontal_space(), icon::hidden()]
                            .width(125)
                            .align_y(Center),
                    )
                    .on_press(Message::ShowSecrets),
                )
            };

            content = content.push(password_field(
                "Password File",
                &entry.pass_id,
                vec![
                    button(row!["Copy", icon::clipboard()].spacing(8).align_y(Center))
                        .on_press(Message::CopyId(entry.clone())),
                ],
            ));
            if let Some(ref line_count) = self.line_count {
                let file_button = if self.show_file {
                    button(
                        row!["Hide file", icon::document()]
                            .spacing(8)
                            .align_y(Center),
                    )
                    .on_press(Message::ShowFile)
                } else {
                    button(
                        row!["Show file", icon::document()]
                            .spacing(8)
                            .align_y(Center),
                    )
                    .on_press(Message::ShowFile)
                };
                content = content.push(password_field(
                    "Number of lines",
                    line_count,
                    vec![file_button],
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
                vec![
                    button(row!["Copy", icon::clipboard()].spacing(8).align_y(Center))
                        .on_press(Message::CopyPassword(entry.clone())),
                ],
            ));
            if let Some(ref login) = self.login {
                content = if self.show_secrets {
                    content.push(password_field(
                        "Login",
                        login,
                        vec![
                            button(row!["Copy", icon::clipboard()].spacing(8).align_y(Center))
                                .on_press(Message::CopyLogin(entry.clone())),
                        ],
                    ))
                } else {
                    content.push(password_field(
                        "Login",
                        "####",
                        vec![
                            button(row!["Copy", icon::clipboard()].spacing(8).align_y(Center))
                                .on_press(Message::CopyLogin(entry.clone())),
                        ],
                    ))
                }
            }
            if let Some(ref otp) = self.otp {
                content = if self.show_secrets {
                    content.push(password_field(
                        "One-time password (OTP)",
                        otp,
                        vec![
                            button(row!["Refresh", icon::refresh()].spacing(8).align_y(Center))
                                .on_press(Message::FetchOtp(entry.clone())),
                            button(row!["Copy", icon::clipboard()].spacing(8).align_y(Center))
                                .on_press(Message::CopyOtp(entry.clone())),
                        ],
                    ))
                } else {
                    content.push(password_field(
                        "One-time password (OTP)",
                        "******",
                        vec![
                            button(row!["Refresh", icon::refresh()].spacing(8).align_y(Center))
                                .on_press(Message::FetchOtp(entry.clone())),
                            button(row!["Copy", icon::clipboard()].spacing(8).align_y(Center))
                                .on_press(Message::CopyOtp(entry.clone())),
                        ],
                    ))
                }
            }

            // Status
            if let Some(ref status) = self.status {
                content = content.push(text(status).color(color!(0x0000ff)));
            }

            // File
            let mut file_area = None;
            if self.show_secrets && self.show_file {
                if let Some(file_contents) = &self.file_contents {
                    file_area = Some(container(
                        container(text(file_contents))
                            .padding(10)
                            .style(container::rounded_box),
                    ));
                }
            }

            column![
                header.spacing(10).padding([10, 10]),
                container(content.spacing(10))
                    .padding(10)
                    .style(container::rounded_box),
            ]
            .push_maybe(file_area)
            .spacing(10)
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
                self.status = Some("Fetching password entry...".to_string());

                let pass_id = entry.pass_id.clone();
                // TODO:The password store directory should not be read each time
                let store_dir = PasswordStore::get_store_dir();
                let file_path = PathBuf::from(store_dir).join(format!("{}.gpg", pass_id));
                async fn helper_function(file_path: PathBuf, pass_id: String) -> (String, String) {
                    match passepartout::decrypt_password_file(&file_path) {
                        Ok(file_contents) => (pass_id, file_contents),
                        Err(_) => (String::new(), String::new()),
                    }
                }
                Action::Run(Task::perform(
                    helper_function(file_path, entry.pass_id),
                    |(id, result)| Message::EntryFetched(id, result),
                ))
            }
            Message::EntryFetched(id, result) => self.update_fields(id, result),
            Message::ShowSecrets => {
                self.show_secrets();
                // Refresh OTP
                if let Some(entry) = &self.entry {
                    let pass_id = entry.pass_id.clone();
                    run_generate_otp(pass_id)
                } else {
                    Action::None
                }
            }
            Message::HideSecrets => {
                self.hide_secrets();
                Action::None
            }
            Message::CopyId(entry) => {
                self.status = Some("Copying password ID to clipboard...".to_string());
                Action::Run(Task::perform(
                    async move { passepartout::copy_id(entry.pass_id).is_ok() },
                    Message::IdCopied,
                ))
            }
            Message::IdCopied(success) => {
                if success {
                    self.status = Some("Password ID copied to clipboard".to_string());
                } else {
                    self.status = Some("Something went wrong".to_string());
                }
                Action::None
            }
            Message::ShowFile => {
                if self.show_file {
                    self.show_file = false;
                } else {
                    self.show_file = true;
                    self.show_secrets = true
                }
                Action::None
            }
            Message::CopyPassword(entry) => {
                self.status = Some("Copying password to clipboard...".to_string());

                let pass_id = entry.pass_id.clone();
                // TODO:The password store directory should not be read each time
                let store_dir = PasswordStore::get_store_dir();
                let file_path = PathBuf::from(store_dir).join(format!("{}.gpg", pass_id));
                Action::Run(Task::perform(
                    async move { passepartout::copy_password(&file_path).is_ok() },
                    Message::PasswordCopied,
                ))
            }
            Message::PasswordCopied(success) => {
                if success {
                    self.status = Some("Password copied to clipboard".to_string());
                } else {
                    self.status = Some("Something went wrong".to_string());
                }
                Action::None
            }
            Message::CopyLogin(entry) => {
                self.status = Some("Copying login to clipboard...".to_string());

                let pass_id = entry.pass_id.clone();
                // TODO:The password store directory should not be read each time
                let store_dir = PasswordStore::get_store_dir();
                let file_path = PathBuf::from(store_dir).join(format!("{}.gpg", pass_id));
                Action::Run(Task::perform(
                    async move { passepartout::copy_login(&file_path).is_ok() },
                    Message::LoginCopied,
                ))
            }
            Message::LoginCopied(success) => {
                if success {
                    self.status = Some("Login copied to clipboard".to_string());
                } else {
                    self.status = Some("Something went wrong".to_string());
                }
                Action::None
            }
            Message::CopyOtp(entry) => {
                self.status = Some("Copying one-time password to clipboard...".to_string());

                // TODO:The password store directory should not be read each time
                let store_dir = PasswordStore::get_store_dir();
                let file_path = PathBuf::from(store_dir).join(format!("{}.gpg", entry.pass_id));
                Action::Run(Task::perform(
                    async move { passepartout::copy_otp(&file_path).is_ok() },
                    Message::OtpCopied,
                ))
            }
            Message::OtpCopied(success) => {
                if success {
                    self.status = Some("One-time password (OTP) copied to clipboard".to_string());
                } else {
                    self.status = Some("Something went wrong".to_string());
                }
                Action::None
            }
            Message::FetchOtp(entry) => {
                self.status = Some("Fetching one-time password...".to_string());

                let pass_id = entry.pass_id.clone();
                run_generate_otp(pass_id)
            }
            Message::OtpFetched(id, otp) => {
                if let Some(ref entry) = self.entry {
                    if entry.pass_id == id {
                        self.otp = Some(otp);
                    }
                    self.status = None;
                } else {
                    self.status = Some("Something went wrong".to_string());
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
        self.status = None;
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
                self.status = Some("Something went wrong".to_string());
                return Action::None;
            }
            self.status = None;
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

                let pass_id = entry.pass_id.clone();
                return run_generate_otp(pass_id);
            }
        }
        Action::None
    }
}

fn run_generate_otp(pass_id: String) -> Action {
    // TODO:The password store directory should not be read each time
    let store_dir = PasswordStore::get_store_dir();
    let file_path = PathBuf::from(store_dir).join(format!("{}.gpg", pass_id));

    async fn helper_function(file_path: PathBuf, pass_id: String) -> (String, String) {
        match passepartout::generate_otp(&file_path) {
            Ok(otp) => (pass_id, otp),
            Err(_) => (String::new(), String::new()),
        }
    }
    Action::Run(Task::perform(
        helper_function(file_path, pass_id),
        |(id, result)| Message::OtpFetched(id, result),
    ))
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
