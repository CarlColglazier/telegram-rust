use hyper::Client;
use hyper::client::response::Response;
use hyper::error::Error;

use std::io::Read;

use rustc_serialize::json;
use rustc_serialize::Decodable;

#[derive(RustcDecodable)]
pub struct ApiResponse<T: Decodable> {
    pub ok: bool,
    pub result: Option<T>,
    pub error_code: Option<u16>,
    pub description: Option<String>,
}

pub fn get(path: &str) -> Result<Response, Error> {
    let mut client = Client::new();
    let url = format!("https://api.telegram.org/{}", path);
    return client.get(&url).send();
}


// TODO: Option -> Result.
pub fn get_body<T: Decodable>(path: &str) -> Option<ApiResponse<T>> {
    let mut response = match get(path) {
        Ok(res) => res,
        Err(_) => return None,
    };
    let mut body = String::new();
    response.read_to_string(&mut body).unwrap();
    match json::decode(&body) {
        Ok(r) => r,
        Err(_) => return None,
    }
}