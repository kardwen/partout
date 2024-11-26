mod events;
mod password_info;
mod password_store;
mod utils;

pub use events::ChannelEvent;
pub use password_info::PasswordInfo;
pub use password_store::PasswordStore;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn asdf() {
        let result = 1;
        assert_eq!(result, 2);
    }
}
