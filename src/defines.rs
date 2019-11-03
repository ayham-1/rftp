mod defines {
    pub enum FTPModes { Active, Passive, Both }
    pub struct PortRange { pub x: i32, pub y: i32 }

    pub struct ServerInfo {
        pub mode: FTPModes,
        pub max_connections: i32,
        pub port_range: PortRange,
        pub allow_anonymous: bool,
        pub log_file: String
    }

    pub struct ClientInfo {
        pub server_name: String,
        pub connect_mode: FTPModes,
        pub username: String,
        pub password: String
    }
}
