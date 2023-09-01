
use tokio::{net::TcpListener, sync::mpsc};
use tokio::sync::Mutex;

use std::error::Error;
use std::net::SocketAddr;
use std::sync::Arc;

use rand::prelude::*;

use super::Client;
use cassowary_plugin_common::{Message, MessageType};

/// Runtime server
#[derive(Debug)]
pub struct Server {
    /// Address the server is listening for TCP connections over
    pub addr: SocketAddr,
    pub clients: Mutex<Vec<Arc<Mutex<Client>>>>,

    receiver: mpsc::Receiver<Message>
}

impl Server {
    pub fn new(addr: SocketAddr) -> Self {
        let (tx, receiver) = mpsc::channel(64);

        let self = Self { addr, receiver, clients: Mutex::new(Vec::new()) };

        // todo: do something with tx

        self
    }

    pub async fn generate_output(&self) -> Result<(), Box<dyn Error>> {
        let mut iteration = 0u32;
        loop {
            {
                let clients = self.clients.lock().await;

                if clients.len() > 0 {
                    let client_id = rand::thread_rng().gen_range(0..clients.len());
                    let client = clients.get(client_id).unwrap().lock().await;
    
                    if let Some(tx) = &client.tx {
                        println!("Sending packet {} to client {}", iteration, client_id);
                        tx.send(
                            Message {
                                id: 0,
                                payload: MessageType::Simple("new packet".into())
                            }
                        ).await.unwrap();
                    }
                    iteration += 1;
                }
            }
    
            let sleep = rand::thread_rng().gen_range(200..3000);
            tokio::time::sleep(std::time::Duration::from_millis(sleep)).await;

            if iteration > 50 {
                break;
            }
        }

        Ok(())
    }

    pub async fn listen_connections<T>(&self, rx: mpsc::Receiver<T>) -> Result<(), Box<dyn Error>> {
        // Create a TCP listener which will listen for incoming
        // connections. This TCP listener is bound to the address we determined
        // above and must be associated with an event loop.
        let listener = TcpListener::bind(&self.addr).await?;
        println!("Listening on: {}", self.addr);

        loop {
            // Asynchronously wait for an inbound socket.
            let (socket, _) = listener.accept().await?;

            // And this is where much of the magic of this server happens. We
            // crucially want all clients to make progress concurrently, rather than
            // blocking one on completion of another. To achieve this we use the
            // `tokio::spawn` function to execute the work in the background.
            //
            // Essentially here we're executing a new task to run concurrently,
            // which will allow all of our clients to be processed concurrently.




            let client = Arc::new(Mutex::new(Client::new(socket)));
            {
                self.clients.lock().await.push( client.clone() );
            }
            client.clone().lock().await.start()?;
        }
    }
}