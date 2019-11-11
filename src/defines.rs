pub mod defines {
    #[derive(Debug)]
    pub enum FTPModes { Active, Passive, Both }
    #[derive(Debug)]
    pub struct PortRange { pub x: i32, pub y: i32 }

    #[derive(Debug)]
    pub struct ServerInfo {
        pub mode: FTPModes,
        pub max_connections: i32,
        pub port_range: PortRange,
        pub allow_anonymous: bool,
        pub log_file: String,
        pub pwd: String
    }

    #[derive(Debug)]
    pub struct ClientInfo {
        pub server_name: String,
        pub connect_mode: FTPModes,
        pub username: String,
        pub password: String
    }
}
