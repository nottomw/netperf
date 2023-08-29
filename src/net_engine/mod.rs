use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering;
use std::sync::Mutex;
use std::time::Duration;

use super::user_config::*;

pub struct NetEngine {
    config: UserConfig,
    // q: Vec<RawBuffer>,std::Vector
}

impl NetEngine {
    pub fn new(config: UserConfig) -> Self {
        println!("{:?}", config);

        return NetEngine { config: config };
    }

    fn data_producer_thread(&mut self) {
        loop {
            // - write data to some queue
            // - wait until data from queue consumed (producer trigger?)
        }
    }

    fn start_client(&self) {
        println!("Client starting...");

        let connectAddr = format!("{}:{}", self.config.ip, self.config.port);
        println!("Connecting to {}...", connectAddr);

        let mut stream = TcpStream::connect(connectAddr).expect("Failed to connect");

        let mut counter = 0;
        loop {
            let mut data_to_send = format!("message from client #{}", counter);

            stream
                .write_all(data_to_send.as_bytes())
                .expect("stream write failed");

            let mut response = [0u8; 1024];
            let n = stream.read(&mut response).expect("Read error");
            let response_text = String::from_utf8_lossy(&response[..n]);
            println!("RX from server: {}", response_text);

            std::thread::sleep(Duration::from_secs(3));
            counter += 1;
        }
    }

    fn handle_client_connection_thread_fn(mut stream: TcpStream) {
        let mut buffer = [0u8; 1024];

        loop {
            match stream.read(&mut buffer) {
                Ok(0) => return, // Connection closed
                Ok(n) => {
                    let request = String::from_utf8_lossy(&buffer[..n]);
                    println!("RX from client: {}", request);

                    stream.write_all(b"ACK").unwrap();
                }
                Err(_) => return,
            }
        }
    }

    fn start_server(&self) {
        println!("Server starting...");

        // TODO: for now forced TCP, should select one from the config

        let listenAddr = format!("127.0.0.1:{}", self.config.port);
        println!("Server listening on {}...", listenAddr);

        let listener = TcpListener::bind(listenAddr).expect("Failed to bind");

        for stream in listener.incoming() {
            match stream {
                Ok(stream) => {
                    std::thread::spawn(|| {
                        Self::handle_client_connection_thread_fn(stream);
                    });
                }

                Err(e) => {
                    eprintln!("Error: {}", e);
                }
            }
        }
    }

    pub fn run(&self) {
        match self.config.appMode {
            AppMode::kClient => {
                self.start_client();
            }

            AppMode::kServer => {
                self.start_server();
            }
        }
    }
}
