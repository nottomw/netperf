use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering;
use std::sync::Mutex;
use std::time::Duration;

use super::user_config::*;
use std::sync::mpsc;

pub struct NetEngine {
    config: UserConfig,
}

// TODO: actually send this in each packet
// Sent in each packet
struct NetEngineMetadata {
    send_timestamp: u64,
    recv_timestamp: u64,
    sequence_no: u64,
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

        println!("Connected, receiving data from server...");

        let mut counter: u64 = 0;
        let mut packet_size_max: usize = 0;
        let mut total_bytes_received: usize = 0;
        let recv_start = std::time::SystemTime::now();
        loop {
            let mut data_from_server: [u8; 4096] = [0; 4096];
            let read_len = stream.read(&mut data_from_server).expect("Read error");

            if read_len > packet_size_max {
                packet_size_max = read_len;
            }

            total_bytes_received += read_len;

            if read_len > 0 {
                // ignore for now...

                // let data_from_server_text: String =
                //     data_from_server.iter().map(|&c| c as char).collect();

                // if let Some(data_from_server_text_prefix) = data_from_server_text.get(0..10) {
                //     println!("RX: {}", data_from_server_text_prefix);
                // }
            } else if read_len == 0 {
                let recv_end = std::time::SystemTime::now();
                let recv_total_time_duration = recv_end
                    .duration_since(recv_start)
                    .expect("duration_since failed");

                let recv_total_time_ms = recv_total_time_duration.as_millis();

                println!("\n\nServer disconnected, calculating stats...");
                println!("\tReceived {} packets...", counter);
                println!("\tMax packet size: {}", packet_size_max);
                println!("\tTotal bytes received: {}", total_bytes_received);
                println!("\tTotal receive time: {} ms", recv_total_time_ms);

                let recv_total_time_s = recv_total_time_duration.as_secs();
                let total_kbytes_received = total_bytes_received / 1024;
                let total_mbytes_received = total_kbytes_received / 1024;
                let bandwidth_bps = total_bytes_received as f64 / recv_total_time_s as f64;
                let bandwidth_kbps = total_kbytes_received as f64 / recv_total_time_s as f64;
                let bandwidth_mbps = total_mbytes_received as f64 / recv_total_time_s as f64;
                println!(
                    "\tBandwidth: {} bps, {} kbps, {} mbps",
                    bandwidth_bps as u32, bandwidth_kbps as u32, bandwidth_mbps as u32
                );
                // TODO: this should be around 20 Gbps?

                break;
            }

            counter += 1;
        }
    }

    fn server_data_producer_thread(channel_sender: std::sync::mpsc::Sender<[char; 4096]>) {
        let packets_to_send = 100_000;
        for i in 1..(packets_to_send + 1) {
            // For now a simple buffer content, maybe something more complicated later...
            let chr = std::char::from_digit(i % 10, 10);
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
                    println!("New client connected, producing data...");

                    let (sender, receiver) = std::sync::mpsc::channel();

                    let thread_builder_1 = std::thread::Builder::new().stack_size(8 * 1024 * 1024); // 8 MB
                    let producer_thread = thread_builder_1.spawn(|| {
                        Self::server_data_producer_thread(sender);
                    });

                    let thread_builder_2 = std::thread::Builder::new().stack_size(8 * 1024 * 1024); // 8 MB
                    let client_handler_thread = thread_builder_2.spawn(|| {
                        Self::server_handle_client_thread(stream, receiver);
                    });

                    producer_thread.unwrap().join().unwrap();
                    client_handler_thread.unwrap().join().unwrap();

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
