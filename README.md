The Telegram crate provides methods for interfacing with the Telegram API.

### Example ###

```rust
use telegram::bot;

// You will need to use your own API key.
let telegram_bot = bot::new("API:KEY");

// Find out some information.
let bot_user = match simple_bot.get_me().send() {
    Some(me) => me,
    None => panic!("Information could not be found."),
};

println!("ID:         {}", bot_user.id);
println!("First name: {}", bot_user.first_name);
println!("Last name:  {}", bot_user.last_name.unwrap_or("None".to_string()));
println!("Username:   {}", bot_user.username.unwrap_or("None".to_string()));
```

[Bot API]: https://core.telegram.org/bots/api "Telegram Bot API"
