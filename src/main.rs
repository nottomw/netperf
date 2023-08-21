use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    println!("Got args:");
    if args.len() > 1 {
        for arg in &args[1..] {
            match arg.as_str() {
                "--client" | "-c" => {
                    println!("client mode...");
                }

                "--server" | "-s" => {
                    println!("server mode...");
                }

                "--tcp" | "-t" => {
                    println!("tcp mode...");
                }

                "--udp" | "-u" => {
                    println!("udp mode...");
                }

                // TODO: packet size
                // TODO: amount of data send/received
                // TODO: test time
                // TODO: statistics selection (throughput, latency, packet loss, jitter, )
                // TODO: fun perf: syn flood, ack flood, other...?
                _ => {
                    panic!("unknown argument provided");
                }
            }
        }
    }
}
