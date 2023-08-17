//! A random data dispatch server with Tokio
//!
//! This server will create a TCP listener, accept connections in a loop, and
//! write back everything that's read off of each TCP connection.
//!
//! Because the Tokio runtime uses a thread pool, each TCP connection is
//! processed concurrently with all other TCP connections across multiple
//! threads.
//!
//! To see this server in action, you can run this in one terminal:
//!
//!     cargo run --example server
//!
//! and in as many other terminals as you want you can run:
//!
//!     cargo run --example client
//!
//! The server will randomly dispatch data to a random connected client

#![warn(rust_2018_idioms)]

use std::env;
use std::error::Error;
use std::sync::Arc;

use cassowary_plugin_runtime::Server;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Allow passing an address to listen on as the first argument of this
    // program, but otherwise we'll just set up our TCP listener on
    // 127.0.0.1:8080 for connections.
    let addr = env::args()
        .nth(1)
        .unwrap_or_else(|| "127.0.0.1:8080".to_string());

    let server = Arc::new( Server::new(addr) );

    let s1 = server.clone();
    let s2 = server.clone();

    let _ = tokio::join!(
        s1.listen_connections(),
        s2.generate_output()
    );

    Ok(())
}
