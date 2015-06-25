
use super::common::request::*;

pub mod types;

use self::types::get::*;
use self::types::post::*;

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

    pub fn get_me(&self) -> Option<types::get::User> {
        let path = format!("bot{}/getMe", self.token);
        return match get_body::<types::get::User>(&path) {
            Some(user) => user.result,
            None => None,
        }
    }

    pub fn send_message(&self, message: SendMessage) -> Option<Message> {
        let path = format!("bot{}/sendMessage?{}",
            self.token,
            message.to_urlencoded_str());
        return match post_body::<Message>(&path) {
            Some(message) => message.result,
            None => None,
        }
    }

    /*
    pub fn get_updates(&self, parameters: types::post::GetUpdates) {
        let perameter_str = perameters.to_urlencoded_str();
        let path = format!("bot{}/getUpdates?{}", self.token, perameter_str);
        post(&path);
    }

    pub fn set_webhook(&self, webhook: &str) {
        let path = format!("bot{}/setWebhook?url={}", self.token, webhook);
        post(&path);
    }
    */
   pub fn send_chat_action(&self, perameters: ChatAction) {
       let perameter_str = perameters.to_urlencoded_str();
       let path = format!("bot{}/sendChatAction?{}", self.token, perameter_str);
       post(&path);
   }
}
