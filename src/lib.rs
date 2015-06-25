/*!
Telegram provides methods for interfacing with the Telegram API.

Currently, only the bot API is supported.

*/
extern crate hyper;
extern crate rustc_serialize;

pub mod bot;
mod common;

use common::*;
