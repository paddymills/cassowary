
/// Message type for communicating with the plugin runtime
#[derive(Debug, Serialize, Deserialize)]
pub struct Message {
    /// message Id number
    // TODO: maybe use uuid for this
    pub id: u32,

    /// massage payload
    pub payload: MessageType,
}

/// Message types be communicated
#[derive(Debug, Serialize, Deserialize)]
pub enum MessageType {
    /// A simple text message
    Simple(String),
}
