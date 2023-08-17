
use tokio::net::TcpListener;
use tokio::sync::Mutex;

use std::error::Error;
use std::sync::Arc;

use rand::prelude::*;

use super::{Client, Message};

#[derive(Debug, Default)]
pub struct Server {
    pub addr: String,
    pub clients: Mutex<Vec<Arc<Mutex<Client>>>>
}

impl Server {
    pub fn new(addr: String) -> Self {
        Self { addr, ..Default::default() }
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
                                id: iteration,
                                msg: "new packet".into()
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

    pub async fn listen_connections(&self) -> Result<(), Box<dyn Error>> {
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
                let client = client.clone();
                let mut clients = self.clients.lock().await;
                clients.push(client);
            }
            client.clone().lock().await.start()?;
        }
    }
}