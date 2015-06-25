pub trait FormUrlEncode {
    fn to_urlencoded_str(&self) -> String;
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
    TYPING,
    UPLOAD_PHOTO,
    RECORD_VIDEO,
    UPLOAD_VIDEO,
    RECORD_AUDIO,
    UPLOAD_AUDIO,
    UPLOAD_DOCUMENT,
    FIND_LOCATION,
}

pub struct ChatAction {
    chat_id: usize,
    action: String,
}

impl ChatAction {
    pub fn new(chat_id: usize, action: Action) -> ChatAction {
        let action_string = match action {
            Action::TYPING => "typing",
            Action::UPLOAD_PHOTO => "upload_photo",
            Action::RECORD_VIDEO => "record_video",
            Action::UPLOAD_VIDEO => "upload_video",
            Action::RECORD_AUDIO => "record_audio",
            Action::UPLOAD_AUDIO => "upload_audio",
            Action::UPLOAD_DOCUMENT => "upload_document",
            Action::FIND_LOCATION => "find_location",
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
