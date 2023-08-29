#[derive(Debug)]
pub enum AppMode {
    kClient,
    kServer,
}

#[derive(Debug)]
pub enum Lay4Mode {
    kTcp,
    kUdp,
}

#[derive(Debug)]
pub struct UserConfig {
    pub appMode: AppMode,
    pub lay4Mode: Lay4Mode,
    pub packetSize: u32,
    pub testTimeSeconds: u64, // 0 - forever, until ctrl-c
    pub port: u32,
    pub ip: String,
}

impl Default for UserConfig {
    fn default() -> Self {
        UserConfig {
            appMode: AppMode::kServer,
            lay4Mode: Lay4Mode::kTcp,
            packetSize: 65535,
            testTimeSeconds: 0,
            port: 9090,
            ip: String::default(),
        }
    }
}
