use super::super::get::User;
use super::super::super::super::common::request::PostRequest;

pub struct GetMe<'a> {
    token: &'a str,
}

impl<'a> GetMe<'a> {
    pub fn new(token: &'a str) -> GetMe<'a> {
        return GetMe {
            token: token,
        };
    }

    pub fn send(&self) -> Option<User> {
        let path = format!("bot{}/getMe", self.token);
        let request = PostRequest::from_string(path);

        let response = match request.send::<User>() {
            Some(res) => res,
            None => return None,
        };
        return response.result;
    }
}
