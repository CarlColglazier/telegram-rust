use super::FormUrlEncode;
use super::super::super::super::common::request::PostRequest;

pub enum Action {
    Typing,
    UploadPhoto,
    RecordVideo,
    UploadVideo,
    RecordAudio,
    UploadAudio,
    UploadDocument,
    FindLocation,
}

pub struct ChatAction<'a> {
    token: &'a str,
    chat_id: usize,
    action: String,
}

impl<'a> ChatAction<'a> {
    pub fn new(token: &'a str, chat_id: usize, action: Action) -> ChatAction<'a> {
        let action_string = match action {
            Action::Typing => "typing",
            Action::UploadPhoto => "upload_photo",
            Action::RecordVideo => "record_video",
            Action::UploadVideo => "upload_video",
            Action::RecordAudio => "record_audio",
            Action::UploadAudio => "upload_audio",
            Action::UploadDocument => "upload_document",
            Action::FindLocation => "find_location",
        };
        return ChatAction {
            token: token,
            chat_id: chat_id,
            action: action_string.to_string(),
        };
    }

    pub fn send(&self) {
        let path = format!("bot{}/sendChatAction?{}", self.token, self.to_urlencoded_str());
        let request = PostRequest::from_string(path);
        request.send::<u8>();
    }
}

impl<'a> FormUrlEncode for ChatAction<'a> {
    fn to_urlencoded_str(&self) -> String {
        return format!("chat_id={}&action={}", self.chat_id, self.action);
    }
}
