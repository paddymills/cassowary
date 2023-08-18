
use crate::Message;

use tokio::io;
use tokio::net::TcpStream;

use std::error::Error;
use std::net::SocketAddr;

/// Cassowary plugin interface
/// 
/// A plugin must implement this trait to ensure proper communication with the
/// plugin runtime. While there is no strict enforcement of the plugin being
/// implemented for the plugin (yet), implementing this trait ensures that proper
/// handling of encoding/decoding of messages over the TCP stream happen.
pub trait CassowaryPlugin {
    /// Error type returned by handlers
    type Error: std::error::Error + 'static;

    /// Listens on the socket for incoming messages
    // TODO: fix this impl (this is copied from client example on 17-8-2023)
    async fn listen(&self, addr: SocketAddr) -> Result<(), Box<dyn Error>> {
        // tcp::connect(&addr, stdin, stdout).await?;
        let stream = TcpStream::connect(addr).await?;

        loop {
            // Wait for the socket to be readable
            stream.readable().await?;

            // Creating the buffer **after** the `await` prevents it from
            // being stored in the async task.
            let mut buf = [0; 4096];

            // Try to read data, this may still fail with `WouldBlock`
            // if the readiness event is a false positive.
            match stream.try_read(&mut buf) {
                Ok(0) => break,
                Ok(_) => {
                    let decoded: Message = bincode::deserialize(&buf).unwrap();
                    self.handle_message(decoded)?;
                }
                Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                    continue;
                }
                Err(e) => {
                    return Err(e.into());
                }
            }
        }

        Ok(())
    }

    /// handles a decoded [`Message`] that was received by [`listen`]
    /// 
    /// ['Message']: crate::Message
    /// ['listen']: crate::listen
    fn handle_message(&self, msg: Message) -> Result<(), Self::Error>;
}