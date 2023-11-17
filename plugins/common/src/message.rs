
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

    /// Message for setting up a plugin's connection
    Setup(ConnectionMessage)
}

/// Messages sent during connection setup phase
#[derive(Debug, Serialize, Deserialize)]
pub enum ConnectionMessage {
    /// Initial message, giving the plugin an name
    Hello { name: String },
}
