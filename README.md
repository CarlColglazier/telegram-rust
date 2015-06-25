The Telegram crate provides methods for interfacing with the Telegram API.

The creation of simple bots using the [Bot API] is supported and designed for
ease of use.

### Example ###

In this simple example, we will connect to the API and find some information
about our bot:

```rust
extern crate telegram;

use telegram::bot;

// You will need to use your own API key.
let telegram_bot = bot::new("API:KEY");

// Find out some information.
let bot_user = match telegram_bot.get_me().send() {
    Some(me) => me,
    None => panic!("Information could not be found."),
};

println!("ID:         {}", bot_user.id);
println!("First name: {}", bot_user.first_name);
println!("Last name:  {}", bot_user.last_name.unwrap_or("None".to_string()));
println!("Username:   {}", bot_user.username.unwrap_or("None".to_string()));
```

### Licensed ###

This crate is licensed under the ISC license. See [LICENSE] for details.

[Bot API]: https://core.telegram.org/bots/api "Telegram Bot API"
[LICENSE]: /LICENSE
