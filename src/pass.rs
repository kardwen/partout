use passepartout::{self, PasswordEvent};

// TODO: this is just a workaround until I have decided on
// on how to structure the shared passepartout library

pub async fn fetch_entry(pass_id: String) -> (String, String) {
    match passepartout::fetch_entry(pass_id.clone(), None) {
        Ok(PasswordEvent::PasswordInfo {
            pass_id,
            file_contents,
        }) => (pass_id, file_contents),
        Ok(_) | Err(_) => (String::new(), String::new()),
    }
}

pub async fn fetch_otp(pass_id: String) -> (String, String) {
    match passepartout::fetch_otp(pass_id.clone(), None) {
        Ok(PasswordEvent::OneTimePassword {
            pass_id,
            one_time_password,
        }) => (pass_id, one_time_password),
        Ok(_) | Err(_) => (String::new(), String::new()),
    }
}
