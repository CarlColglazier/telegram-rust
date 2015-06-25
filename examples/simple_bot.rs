extern crate telegram;

use telegram::bot::Bot;

mod shared;

fn main() {
    let simple_bot = Bot::new(&shared::read_key());

    let bot_user = match simple_bot.get_me().send() {
        Some(me) => me,
        None => panic!(":("),
    };
    println!("ID:         {}", bot_user.id);
    println!("First name: {}", bot_user.first_name);
    println!("Last name:  {}", bot_user.last_name.unwrap_or("None".to_string()));
    println!("Username:   {}", bot_user.username.unwrap_or("None".to_string()));
}
