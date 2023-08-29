#![allow(non_snake_case)] // shut up about snake/camel for now
#![allow(non_camel_case_types)]

mod double_buffer;
mod net_engine;
mod user_config;

use clap;

use net_engine::*;
use user_config::*;

fn main() {
    let mut config = UserConfig::default();

    let matches = clap::Command::new("netperf") //
        .version("1.0")
        .arg_required_else_help(true)
        .arg(
            clap::arg!(-c --client "client mode") //
                .conflicts_with("server"),
        )
        .arg(
            clap::arg!(-s --server "server mode") //
                .conflicts_with("client"),
        )
        .arg(
            clap::arg!(-t --tcp "TCP") //
                .conflicts_with("udp"),
        )
        .arg(
            clap::arg!(-u --udp "UDP") //
                .conflicts_with("tcp"),
        )
        .arg(
            clap::arg!(-p --port <PORT> "port to use") //
                .value_parser(clap::value_parser!(u32))
                .action(clap::ArgAction::Set),
        )
        .arg(
            clap::arg!(-i --ip <IP> "ip address to use") //
                .value_parser(clap::value_parser!(String))
                .action(clap::ArgAction::Set)
                .conflicts_with("server"), // or should bind to a specific address?
        )
        .arg(
            clap::arg!(-z --packetsize <SIZE> "size of the sent packet [bytes]") // could become format: 1B, 2KB, ...
                .value_parser(clap::value_parser!(u32))
                .action(clap::ArgAction::Set)
                .conflicts_with("server"),
        )
        .arg(
            clap::arg!(--time <TIME> "define for how long should the test run [seconds]") // could become format: 1h20m30s
                .value_parser(clap::value_parser!(u64))
                .action(clap::ArgAction::Set),
        )
        .get_matches();

    let clientMode = matches.get_flag("client");
    let serverMode = matches.get_flag("server");

    if clientMode {
        config.appMode = AppMode::kClient;
    }

    if serverMode {
        config.appMode = AppMode::kServer;
    }

    let tcpMode = matches.get_flag("tcp");
    let udpMode = matches.get_flag("udp");

    if tcpMode {
        config.lay4Mode = Lay4Mode::kTcp;
    }

    if udpMode {
        config.lay4Mode = Lay4Mode::kUdp;
    }

    if matches.contains_id("ip") {
        config.port = *(matches.get_one::<u32>("port").expect("port missing?"));
    }

    if matches.contains_id("ip") {
        config.ip = matches
            .get_one::<String>("ip")
            .expect("ip required")
            .clone();
    }

    if matches.contains_id("packetsize") {
        config.packetSize = *(matches
            .get_one::<u32>("packetsize")
            .expect("packet size missing?"));
    }

    if matches.contains_id("time") {
        config.testTimeSeconds = *(matches.get_one::<u64>("time").expect("time missing?"));
    }

    let engine = NetEngine::new(config);
    engine.run();
}
