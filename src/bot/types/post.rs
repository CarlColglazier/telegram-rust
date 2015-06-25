pub trait FormUrlEncode {
    fn to_urlencoded_str(&self) -> String;
}

pub struct SendMessage {
    chat_id: usize,
    text: String,
    // TODO: disable_web_page_preview
    // TODO: reply_to_message_id
    // TODO: reply_markup
}

impl SendMessage {
    pub fn new(chat_id: usize, text: &str) -> SendMessage {
        return SendMessage {
            chat_id: chat_id,
            text: text.to_string(),
        }
    }
}

impl FormUrlEncode for SendMessage {
    fn to_urlencoded_str(&self) -> String {
        return format!("chat_id={}&text={}", self.chat_id, self.text);
    }
}

pub struct GetUpdates {
    offset: Option<usize>,
    limit: Option<usize>,
    timeout: Option<usize>,
}

impl FormUrlEncode for GetUpdates {
    fn to_urlencoded_str(&self) -> String {
        let mut url_value_pairs: Vec<(String, String)> = Vec::new();
        match self.offset {
            Some(offset) => {
                url_value_pairs.push(("offset".to_string(), offset.to_string()));
            },
            None => {},
        }
        match self.limit {
            Some(limit) => {
                url_value_pairs.push(("limit".to_string(), limit.to_string()));
            },
            None => {},
        }
        match self.timeout {
            Some(timeout) => {
                url_value_pairs.push(("timeout".to_string(), timeout.to_string()));
            },
            None => {},
        }
        let mut url_values: Vec<String> = Vec::new();
        for field in url_value_pairs.into_iter() {
            url_values.push(format!("{}={}", field.0, field.1));
        }
        let string = url_values.connect("&");
        return string;
    }
}

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

pub struct ChatAction {
    chat_id: usize,
    action: String,
}

impl ChatAction {
    pub fn new(chat_id: usize, action: Action) -> ChatAction {
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
            chat_id: chat_id,
            action: action_string.to_string(),
        };
    }
}

impl FormUrlEncode for ChatAction {
    fn to_urlencoded_str(&self) -> String {
        return format!("chat_id={}&action={}", self.chat_id, self.action);
    }
}
