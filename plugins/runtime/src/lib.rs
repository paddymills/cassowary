
#[macro_use] extern crate serde;

mod server;
mod client;

pub use server::Server;
pub use client::Client;

#[derive(Debug, Serialize, Deserialize)]
pub struct Message {
    pub id: u32,
    pub msg: String
}
