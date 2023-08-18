

/// Message type for communicating with the plugin runtime
// TODO: this should probably be an enum of different message types
#[derive(Debug, Serialize, Deserialize)]
pub struct Message {
    /// message Id number
    // TODO: maybe use uuid for this
    pub id: u32,

    /// message payload
    pub msg: String
}
