use hyper::Client;
use hyper::client::response::Response;
use hyper::error::Error;

use std::io::Read;

use rustc_serialize::json;
use rustc_serialize::Decodable;
use rustc_serialize::Encodable;

#[derive(RustcDecodable)]
pub struct ApiResponse<T: Decodable> {
    pub ok: bool,
    pub result: Option<T>,
    pub error_code: Option<u16>,
    pub description: Option<String>,
}

fn format_path(path: &str) -> String {
    format!("https://api.telegram.org/{}", path)
}

pub fn get(path: &str) -> Result<Response, Error> {
    let mut client = Client::new();
    let url = format_path(path);
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

pub fn post(path: &str) -> Result<Response, Error> {
    let mut client = Client::new();
    let url = format_path(path);
    return client.post(&url).send();
}

pub fn post_body<T: Decodable>(path: &str) -> Option<ApiResponse<T>> {
    let mut response = match post(path) {
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
