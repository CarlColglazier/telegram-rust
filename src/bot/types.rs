#[derive(RustcDecodable)]
pub struct User {
    pub id: usize,
    pub first_name: String,
    pub last_name: Option<String>,
    pub username: Option<String>,
}
