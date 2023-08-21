#![allow(non_snake_case)] // shut up about snake/camel for now
#![allow(non_camel_case_types)]

use std::env;

#[derive(Debug)]
enum AppMode {
    kClient,
    kServer,
}

#[derive(Debug)]
enum Lay4Mode {
    kTcp,
    kUdp,
}

#[derive(Debug)]
struct UserConfig {
    appMode: AppMode,
    lay4Mode: Lay4Mode,
    packetSize: u32,
    testTimeSeconds: u64, // 0 - forever, until ctrl-c
}

impl Default for UserConfig {
    fn default() -> Self {
        UserConfig {
            appMode: AppMode::kClient,
            lay4Mode: Lay4Mode::kTcp,
            packetSize: 65535,
            testTimeSeconds: 0,
        }
    }
}

struct NetEngine {
    config: UserConfig,
}

impl NetEngine {
    pub fn new(config: UserConfig) -> Self {
        println!("{:?}", config);

        return NetEngine { config };
    }

    pub fn run(&self) {
        println!("Net engine should run, but for now this is just a dummy function...");
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let mut config = UserConfig::default();

    // TODO: arg parser needed
    println!("Got args:");
    if args.len() > 1 {
        for arg in &args[1..] {
            match arg.as_str() {
                "--client" | "-c" => {
                    println!("client mode...");
                    // TODO: requires target IP adress and port
                    config.appMode = AppMode::kClient;
                }

                "--server" | "-s" => {
                    println!("server mode...");
                    // TODO: requires port
                    config.appMode = AppMode::kServer;
                }

                "--tcp" | "-t" => {
                    println!("tcp mode...");
                    config.lay4Mode = Lay4Mode::kTcp;
                }

                "--udp" | "-u" => {
                    println!("udp mode...");
                    config.lay4Mode = Lay4Mode::kUdp;
                }

                // TODO: packet size
                // TODO: amount of data send/received
                // TODO: test time (format: 1h23m45s)
                // TODO: statistics selection (throughput, latency, packet loss, jitter, )
                // TODO: fun perf: syn flood, ack flood, other...?
                _ => {
                    panic!("unknown argument provided");
                }
            }
        }
    }

    let engine = NetEngine::new(config);
    engine.run();
}
