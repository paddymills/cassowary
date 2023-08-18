
use tokio::io::{AsyncReadExt, AsyncWriteExt, ReadHalf, WriteHalf};
use tokio::net::TcpStream;
use tokio::sync::{mpsc, Mutex};

use std::error::Error;
use std::sync::Arc;

use cassowary_plugin_common::{Message, MessageType};

#[derive(Debug)]
pub struct Client {
    pub tx: Option<mpsc::Sender<Message>>,
    read:  Arc<Mutex<ReadHalf<TcpStream>>>,
    write: Arc<Mutex<WriteHalf<TcpStream>>>,
}

impl Client {
    pub fn new(socket: TcpStream) -> Self {
        let (read_stream, write_stream) = tokio::io::split(socket);
        let read  = Arc::new(Mutex::new(read_stream));
        let write = Arc::new(Mutex::new(write_stream));

        Self { tx: None, read, write }
    }

    pub fn start(&mut self) -> Result<(), Box<dyn Error>> {
        let (tx, mut rx) = mpsc::channel(16);
        self.tx = Some( tx.clone() );

        let read = self.read.clone();
        tokio::spawn(async move {
            let mut buf = vec![0; 1024];

            // In a loop, read data from the socket and write the data back.
            loop {
                let n = read
                    .lock()
                        .await
                    .read(&mut buf)
                    .await
                    .expect("failed to read data from socket");

                if n == 0 {
                    return;
                }

                println!("received data: {}", std::str::from_utf8(&buf[0..n-1]).unwrap());

                tx.send(
                    Message {
                        id: 0,
                        payload: MessageType::Simple(String::from_utf8(buf.to_vec()).unwrap())
                    }
                ).await.unwrap();
            }
        });

        let write = self.write.clone();
        tokio::spawn(async move {
            while let Some(buf) = rx.recv().await {
                write
                    .lock()
                        .await
                    .write_all( &bincode::serialize(&buf).unwrap() )
                        .await
                    .expect("failed to write data to socket");
            }
        });

        Ok(())
    }
}

