use arboard::Clipboard;
use std::{
    collections::HashMap,
    env,
    ffi::OsStr,
    fs, io,
    path::{Path, PathBuf},
    process::{Command, Stdio},
    sync::mpsc::Sender,
    thread::JoinHandle,
};

use crate::{events::ChannelEvent, password_info::PasswordInfo, utils::run_once};

pub struct PasswordStore {
    pub passwords: Vec<PasswordInfo>,
    event_tx: Sender<ChannelEvent>,
    ops_map: HashMap<String, (JoinHandle<()>, String)>,
    pub clipboard: Option<Clipboard>,
}

impl PasswordStore {
    pub fn new(event_tx: Sender<ChannelEvent>) -> Self {
        let dir = Self::get_store_dir();
        let mut passwords = Self::get_password_infos(&dir);
        passwords.sort_by_key(|element| element.pass_id.clone());
        Self {
            passwords,
            event_tx,
            ops_map: HashMap::new(),
            clipboard: Clipboard::new().ok(),
        }
    }

    pub fn get_store_dir() -> PathBuf {
        let home = dirs::home_dir().expect("could not determine home directory");
        if let Some(store_path) = env::var_os("PASSWORD_STORE_DIR") {
            let path = PathBuf::from(store_path);
            if path.is_absolute() {
                return path;
            } else if let Ok(relative_to_home) = path
                .strip_prefix("$HOME")
                .or_else(|_| path.strip_prefix("~"))
            {
                return home.join(relative_to_home);
            };
        }
        home.join(".password-store")
    }

    pub fn get_password_infos(store_dir: &Path) -> Vec<PasswordInfo> {
        Self::read_store_dir(store_dir)
            .unwrap_or_default()
            .iter()
            .filter_map(|path| {
                let relative_path = path.strip_prefix(store_dir).expect("prefix does exist");
                match path.metadata() {
                    Ok(metadata) => Some(PasswordInfo::new(relative_path, metadata)),
                    Err(_) => None,
                }
            })
            .collect()
    }

    pub fn read_store_dir(dir: &Path) -> io::Result<Vec<PathBuf>> {
        let mut result = Vec::new();

        fn visit_dir(dir: &Path, result: &mut Vec<PathBuf>) -> io::Result<()> {
            for entry in fs::read_dir(dir)? {
                let path = entry?.path();
                if path.is_dir() {
                    visit_dir(&path, result)?;
                } else if path.is_file() && path.extension().is_some_and(|ext| ext == "gpg") {
                    result.push(path);
                }
            }
            Ok(())
        }

        visit_dir(dir, &mut result)?;
        Ok(result)
    }

    pub fn copy_pass_id(&mut self, pass_id: String) {
        let tx = self.event_tx.clone();
        if let Some(ref mut clipboard) = self.clipboard {
            match clipboard.set_text(pass_id) {
                Ok(()) => {
                    let message = "Password file identifier copied to clipboard".into();
                    tx.send(ChannelEvent::Status(message))
                        .expect("receiver deallocated");
                }
                Err(e) => {
                    let message = format!("Failed to copy password file identifier: {e:?}");
                    tx.send(ChannelEvent::Status(message))
                        .expect("receiver deallocated");
                }
            }
        } else {
            let message = String::from("✗ Clipboard not available");
            tx.send(ChannelEvent::Status(message))
                .expect("receiver deallocated");
        }
    }

    pub fn copy_password(&mut self, pass_id: String) {
        let tx = self.event_tx.clone();

        fn pass_fn(pass_id: String, tx: Sender<ChannelEvent>) {
            let message = String::from("⧗ (pass) Copying password...");
            tx.send(ChannelEvent::Status(message))
                .expect("receiver deallocated");
            let status = Command::new("pass")
                .arg(OsStr::new(&pass_id))
                .arg("--clip")
                .stderr(Stdio::null())
                .stdout(Stdio::null())
                .status()
                .expect("failed to execute process");
            let message = if status.success() {
                "Password copied to clipboard, clears after 45 seconds".to_string()
            } else {
                format!("(pass) {status}")
            };
            let status_event = ChannelEvent::Status(message);
            tx.send(status_event).expect("receiver deallocated");
        }

        run_once(
            &mut self.ops_map,
            "pass_copy_password".into(),
            pass_id.clone(),
            move || pass_fn(pass_id, tx),
        );
    }

    pub fn copy_login(&mut self, pass_id: String) {
        let tx = self.event_tx.clone();

        fn pass_fn(pass_id: String, tx: Sender<ChannelEvent>) {
            let message = String::from("⧗ (pass) Copying login...");
            tx.send(ChannelEvent::Status(message))
                .expect("receiver deallocated");
            let status = Command::new("pass")
                .arg(OsStr::new(&pass_id))
                .arg("--clip=2")
                .stderr(Stdio::null())
                .stdout(Stdio::null())
                .status()
                .expect("failed to execute process");
            let message = if status.success() {
                "Login copied to clipboard, clears after 45 seconds".to_string()
            } else {
                format!("✗ (pass) {status}")
            };
            let status_event = ChannelEvent::Status(message);
            tx.send(status_event).expect("receiver deallocated");
        }

        run_once(
            &mut self.ops_map,
            "pass_copy_login".into(),
            pass_id.clone(),
            move || pass_fn(pass_id, tx),
        );
    }

    pub fn copy_one_time_password(&mut self, pass_id: String) {
        let tx = self.event_tx.clone();

        fn pass_fn(pass_id: String, tx: Sender<ChannelEvent>) {
            let message = String::from("⧗ (pass) Copying one-time password...");
            tx.send(ChannelEvent::Status(message))
                .expect("receiver deallocated");
            let status = Command::new("pass")
                .arg("otp")
                .arg("code")
                .arg(OsStr::new(&pass_id))
                .arg("--clip")
                .stderr(Stdio::null())
                .stdout(Stdio::null())
                .status()
                .expect("failed to execute process");
            let message = if status.success() {
                "One-time password copied to clipboard".to_string()
            } else {
                format!("✗ (pass) {status}")
            };
            let status_event = ChannelEvent::Status(message);
            tx.send(status_event).expect("receiver deallocated");
        }

        run_once(
            &mut self.ops_map,
            "pass_otp_copy".into(),
            pass_id.clone(),
            move || pass_fn(pass_id, tx),
        );
    }

    pub fn fetch_one_time_password(&mut self, pass_id: String) {
        let tx = self.event_tx.clone();

        fn pass_fn(pass_id: String, tx: Sender<ChannelEvent>) {
            let message = String::from("⧗ (pass) Fetching one-time password...");
            tx.send(ChannelEvent::Status(message))
                .expect("receiver deallocated");
            let output = Command::new("pass")
                .arg("otp")
                .arg("code")
                .arg(OsStr::new(&pass_id))
                .output()
                .expect("failed to execute process");
            if output.status.success() {
                let one_time_password = String::from_utf8_lossy(&output.stdout).to_string();
                tx.send(ChannelEvent::OneTimePassword {
                    pass_id,
                    one_time_password,
                })
                .expect("receiver deallocated");
                tx.send(ChannelEvent::ResetStatus)
                    .expect("receiver deallocated");
            } else {
                let message = format!("✗ (pass) {}", String::from_utf8_lossy(&output.stderr));
                tx.send(ChannelEvent::Status(message))
                    .expect("receiver deallocated");
            }
        }

        run_once(
            &mut self.ops_map,
            "pass_otp_fetch".into(),
            pass_id.clone(),
            move || pass_fn(pass_id, tx),
        );
    }

    pub fn fetch_pass_details(&mut self, pass_id: String) {
        let tx = self.event_tx.clone();

        fn pass_fn(pass_id: String, tx: Sender<ChannelEvent>) {
            let message = String::from("⧗ (pass) Fetching password entry...");
            tx.send(ChannelEvent::Status(message))
                .expect("receiver deallocated");
            let output = Command::new("pass")
                .arg(OsStr::new(&pass_id))
                .output()
                .expect("failed to execute process");
            if output.status.success() {
                let file_contents = String::from_utf8_lossy(&output.stdout).to_string();
                tx.send(ChannelEvent::PasswordInfo {
                    pass_id,
                    file_contents,
                })
                .expect("receiver deallocated");
                tx.send(ChannelEvent::ResetStatus)
                    .expect("receiver deallocated");
            } else {
                let message = format!("✗ (pass) {}", String::from_utf8_lossy(&output.stderr));
                tx.send(ChannelEvent::Status(message))
                    .expect("receiver deallocated");
            };
        }

        run_once(
            &mut self.ops_map,
            "pass_show".into(),
            pass_id.clone(),
            move || pass_fn(pass_id, tx.clone()),
        );
    }
}
