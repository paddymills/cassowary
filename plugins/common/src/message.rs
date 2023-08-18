

/// Message type for communicating with the plugin runtime
#[derive(Debug, Serialize, Deserialize)]
pub struct Message {
    pub id: u32,
    pub msg: String
}
