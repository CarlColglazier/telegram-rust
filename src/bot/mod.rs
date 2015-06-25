/*!
Interfaces for creating and managing Telegram bots.

The functions and structures in this module are designed to match the Telegram
[Bot API].

[Bot API]: https://core.telegram.org/bots/api "Telegram Bot API"
*/


/// Structures for receiving and sending data.
pub mod types;

use self::types::post::{
    GetMe,
    Action,
    ChatAction
};

/// Bot represents a Telegram Bot.
///
/// Each request is sent by creating a constructor and then
///
/// # Examples
///
/// ```no_run
/// # KEY: &'static str = "KEY";
/// // KEY is a static string slice.
/// let bot = Bot::new(KEY);
/// let bot_id = bot.get_me().send();
/// ```
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

    /// Get information about the bot.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # let bot = Bot::new("_");
    /// bot.get_me().send();
    /// ```
    pub fn get_me(&self) -> GetMe {
        return GetMe::new(&self.token);
    }

    /// Send a chat actions
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # let bot = Bot::new("_");
    /// bot.chat_action(Action::Typing).send();
    /// ```
   pub fn chat_action(&self, chat_id: usize, action: Action) -> ChatAction {
       return ChatAction::new(&self.token, chat_id, action);
   }
}
