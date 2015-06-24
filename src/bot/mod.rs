mod types;

use super::common::request::get_body;

pub struct Bot {
    token: String,
}

impl Bot {

    /// Create a new Telegram bot.
    pub fn new(token: &str) -> Bot {
        return Bot {
            token: token.to_string(),
        }
    }

    pub fn get_me(&self) -> Option<types::User> {
        let path = format!("bot{}/getMe", self.token);
        return match get_body::<types::User>(&path) {
            Some(user) => user.result,
            None => None,
        }
    }
}
