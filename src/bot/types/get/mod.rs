#[derive(RustcDecodable)]
pub struct Update {
    pub update_id: usize,
    pub message: Option<Message>,
}

#[derive(RustcDecodable)]
pub struct User {
    pub id: usize,
    pub first_name: String,
    pub last_name: Option<String>,
    pub username: Option<String>,
}

#[derive(RustcDecodable)]
pub struct GroupChat {
    pub id: usize,
    pub title: String,
}

#[derive(RustcDecodable)]
pub struct Message {
    pub message_id: usize,
    pub from: User,
    pub date: usize,
    // chat
    pub forward_from: Option<User>,
    pub forward_date: Option<usize>,
    pub reply_to_message: Option<Box<Message>>,
    // TODO
}

#[derive(RustcDecodable)]
pub struct PhotoSize {
    pub file_id: String,
    pub width: usize,
    pub height: usize,
    pub file_size: Option<usize>,
}

#[derive(RustcDecodable)]
pub struct Audio {
    pub file_id: String,
    pub duration: usize,
    pub mime_type: Option<String>,
    pub file_size: Option<usize>,
}

#[derive(RustcDecodable)]
pub struct Document {
    pub file_id: String,
    pub thumb: PhotoSize,
    pub file_name: Option<String>,
    pub mime_type: Option<String>,
    pub file_size: Option<usize>,
}

#[derive(RustcDecodable)]
pub struct Sticker {
    pub file_id: String,
    pub width: usize,
    pub height: usize,
    pub thumb: PhotoSize,
    pub file_size: Option<usize>,
}

#[derive(RustcDecodable)]
pub struct Video {
    pub file_id: String,
    pub width: usize,
    pub height: usize,
    pub duration: usize,
    pub thumb: PhotoSize,
    pub mime_type: Option<String>,
    pub file_size: Option<usize>,
    pub caption: Option<String>,
}

#[derive(RustcDecodable)]
pub struct Contact {
    pub phone_number: String,
    pub first_name: String,
    pub last_name: Option<String>,
    pub user_id: Option<String>,
}


#[derive(RustcDecodable)]
pub struct Location {
    pub longitude: f64,
    pub latitude: f64,
}


#[derive(RustcDecodable)]
pub struct UserProfilePhotos {
    pub total_count: usize,
    pub photos: Vec<Vec<PhotoSize>>,
}

#[derive(RustcDecodable)]
pub struct ReplyKeyboardMarkup {
    pub keyboard: Vec<Vec<String>>,
    pub resize_keyboard: Option<bool>,
    pub one_time_keyboard: Option<bool>,
    pub selective: Option<bool>,
}

#[derive(RustcDecodable)]
pub struct ReplyKeyboardHide {
    pub hide_keyboard: bool,
    pub selective: Option<bool>,
}

#[derive(RustcDecodable)]
pub struct ForceReply {
    pub force_reply: bool,
    pub selective: Option<bool>,
}
