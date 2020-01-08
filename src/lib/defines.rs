pub mod defines {
    use crate::db::*;
    use std::net::{TcpStream};
    use net2::TcpBuilder;

    #[derive(Debug)]
    pub enum FTPModes { Active, Passive, Both }
    impl Default for FTPModes {
        fn default() -> Self { FTPModes::Both }
    }
    #[derive(Default, Debug)]
    pub struct PortRange { pub x: i32, pub y: i32 }
    #[derive(Debug)]
    pub enum FTPTypes { ASCII, BINARY }
    impl Default for FTPTypes {
        fn default() -> Self { FTPTypes::ASCII }
    }

    #[derive(Default, Debug)]
    pub struct ServerInfo {
        pub mode: FTPModes,
        pub max_connections: i32,
        pub port_range: PortRange,
        pub allow_anonymous: bool,
        pub log_file: String,
        pub pwd: String
    }

    #[derive(Default, Debug)]
    pub struct ClientInfo {
        pub server_name: String,
        pub connect_mode: FTPModes,
        pub username: String,
        pub password: String
    }

   #[derive(Debug)]
    pub struct ClientConnection {
        pub user: db::User,
        pub connect_mode: FTPModes,
        pub data_type: FTPTypes,
        pub data_ip: String,
        pub data_port: i32,
        pub data_conc: TcpStream,
        pub cwd: String,
        pub is_data_up: bool,
        pub is_user_logged: bool, 
        pub is_closing: bool,
        pub is_requesting_login: bool,
        pub is_anon: bool,
    }
    impl Default for ClientConnection {
        fn default() -> Self {
            ClientConnection {
                data_conc: TcpBuilder::new_v4().unwrap().to_tcp_stream().unwrap(),
                user: db::User::default(),
                connect_mode: FTPModes::default(),
                data_type: FTPTypes::default(),
                data_ip: String::default(),
                data_port: i32::default(),
                cwd: String::default(),
                is_data_up: bool::default(),
                is_user_logged: bool::default(),
                is_closing: bool::default(),
                is_requesting_login: bool::default(),
                is_anon: bool::default(),
            }
        } 
    }
    #[derive(Default, Debug)]
    pub struct ServerStatus {
        pub is_command_port_open: bool,
        pub active_connections: i32
    }
}
