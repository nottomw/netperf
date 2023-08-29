use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering;
use std::sync::Mutex;
use std::time::Duration;

use super::double_buffer::*;
use super::user_config::*;
use std::sync::mpsc;

pub struct NetEngine {
    config: UserConfig,
}

impl NetEngine {
    pub fn new(config: UserConfig) -> Self {
        println!("{:?}", config);

        return NetEngine { config: config };
    }

    fn client_start(&self) {
        println!("Client starting...");

        let connectAddr = format!("{}:{}", self.config.ip, self.config.port);
        println!("Connecting to {}...", connectAddr);

        let mut stream = TcpStream::connect(connectAddr).expect("Failed to connect");

        let mut counter = 0;
        let mut packet_size_max = 0;
        loop {
            let mut data_from_server: [u8; 4096] = [0; 4096];
            let read_len = stream.read(&mut data_from_server).expect("Read error");

            if read_len > packet_size_max {
                packet_size_max = read_len;
            }

            if read_len > 0 {
                // let data_from_server_text = String::from_utf8_lossy(&data_from_server[..read_len]);
                let data_from_server_text: String =
                    data_from_server.iter().map(|&c| c as char).collect();

                println!("RX: {}", data_from_server_text);
            } else if read_len == 0 {
                println!("Server disconnected, calculating stats...");
                println!("\tReceived {} packets...", counter);
                println!("\tMax packet size: {}", packet_size_max);
                // TODO: print stats here?
                break;
            }

            counter += 1;
        }
    }

    fn server_data_producer_thread(channel_sender: std::sync::mpsc::Sender<[char; 4096]>) {
        for i in 1..10 {
            // For now a simple buffer content, maybe something more complicated later...
            let chr = std::char::from_digit(i, 10);
            if let Some(character) = chr {
                let buf_to_send: [char; 4096] = [character; 4096];
                channel_sender
                    .send(buf_to_send)
                    .expect("failed to send through channel");
            }
        }
    }

    fn server_handle_client_thread(
        mut stream: TcpStream,
        channel_receiver: std::sync::mpsc::Receiver<[char; 4096]>,
    ) {
        loop {
            let data_to_send_res = channel_receiver.recv();
            match data_to_send_res {
                Ok(data_to_send) => {
                    println!("Sending data: {}", data_to_send[0]);

                    let data_to_send_bytes: Vec<u8> =
                        data_to_send.iter().map(|&c| c as u8).collect();

                    stream
                        .write_all(&data_to_send_bytes)
                        .expect("write_all failed");
                }

                Err(err) => {
                    println!("Error: {}, stopping client data send...", err);
                    break;
                }
            }
        }
    }

    fn server_start(&self) {
        println!("Server starting...");

        // TODO: for now forced TCP, should select one from the config

        let listenAddr = format!("127.0.0.1:{}", self.config.port);
        println!("Server listening on {}...", listenAddr);

        let listener = TcpListener::bind(listenAddr).expect("Failed to bind");

        for stream in listener.incoming() {
            match stream {
                Ok(stream) => {
                    let (sender, receiver) = std::sync::mpsc::channel();

                    let producer_thread = std::thread::spawn(|| {
                        Self::server_data_producer_thread(sender);
                    });

                    let client_handler_thread = std::thread::spawn(|| {
                        Self::server_handle_client_thread(stream, receiver);
                    });

                    producer_thread.join().unwrap();
                    client_handler_thread.join().unwrap();

                    // break if the threads joined - all done
                    break;
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
                self.client_start();
            }

            AppMode::kServer => {
                self.server_start();
            }
        }
    }
}
