use std::{sync::mpsc, thread::sleep, time::Duration};

use passepartout::{ChannelEvent, PasswordStore};

// TODO: this is just a workaround until I have decided on
// on how to structure the shared passepartout library

pub async fn fetch_entry(pass_id: String) -> (String, String) {
    let (event_tx, event_rx) = mpsc::channel();
    PasswordStore::new(event_tx).fetch_pass_details(pass_id.clone());
    loop {
        let event = event_rx.recv().ok();
        match event {
            Some(ChannelEvent::PasswordInfo {
                pass_id,
                file_contents,
            }) => return (pass_id, file_contents),
            Some(ChannelEvent::Status(status)) => {
                if status.starts_with("✗") {
                    break;
                }
            }
            _ => (),
        }
    }
    (String::new(), String::new())
}

pub async fn fetch_otp(pass_id: String) -> (String, String) {
    let (event_tx, event_rx) = mpsc::channel();
    PasswordStore::new(event_tx).fetch_otp(pass_id.clone());
    loop {
        let event = event_rx.recv().ok();
        match event {
            Some(ChannelEvent::OneTimePassword {
                pass_id,
                one_time_password,
            }) => return (pass_id, one_time_password),
            Some(ChannelEvent::Status(status)) => {
                if status.starts_with("✗") {
                    break;
                }
            }
            _ => (),
        }
    }
    (String::new(), String::new())
}

pub async fn copy_id(pass_id: String) -> bool {
    // TODO: doesn't work because clipboard moves out of scope
    let (event_tx, _event_rx) = mpsc::channel();
    PasswordStore::new(event_tx).copy_id(pass_id.clone());
    sleep(Duration::from_secs(5));
    true
}

pub async fn copy_password(pass_id: String) -> bool {
    let (event_tx, _event_rx) = mpsc::channel();
    PasswordStore::new(event_tx).copy_password(pass_id.clone());
    sleep(Duration::from_secs(5));
    true
}

pub async fn copy_login(pass_id: String) -> bool {
    let (event_tx, _event_rx) = mpsc::channel();
    PasswordStore::new(event_tx).copy_login(pass_id.clone());
    sleep(Duration::from_secs(5));
    true
}

pub async fn copy_otp(pass_id: String) -> bool {
    let (event_tx, _event_rx) = mpsc::channel();
    PasswordStore::new(event_tx).copy_otp(pass_id.clone());
    sleep(Duration::from_secs(5));
    true
}
