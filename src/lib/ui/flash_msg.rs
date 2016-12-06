use rustc_serialize::json::{ToJson, Json};
use std::collections::BTreeMap;

#[allow(dead_code)]
pub struct FlashMessage {
    pub msg_type: MessageType,
    pub header: String,
    pub msg: String,
}

#[allow(dead_code)]
pub enum MessageType {
    Normal,
    Info,
    Warning,
    Success,
    Error,
}

impl ToJson for MessageType {
    fn to_json(&self) -> Json {
        match self {
            &MessageType::Normal => "".to_json(),
            &MessageType::Info => "info".to_json(),
            &MessageType::Warning => "warning".to_json(),
            &MessageType::Success => "success".to_json(),
            &MessageType::Error => "error".to_json(),
        }
    }
}

impl ToJson for FlashMessage {
    fn to_json(&self) -> Json {
        let mut m: BTreeMap<String, Json> = BTreeMap::new();
        m.insert("msg_type".to_string(), self.msg_type.to_json());
        m.insert("header".to_string(), self.header.to_json());
        m.insert("msg".to_string(), self.msg.to_json());
        m.to_json()
    }
}
