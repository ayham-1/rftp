pub mod defines {
    use serde::{Deserialize, Serialize};
    use std::net::{TcpStream};
    use net2::TcpBuilder;
    use std::error::Error;
    use std::fmt;

    #[derive(PartialEq, Debug)]
    pub enum FTPModes { Active, Passive, Both }
    impl Default for FTPModes {
        fn default() -> Self { FTPModes::Both }
    }

    #[derive(PartialEq, Debug)]
    pub enum FTPTypes { ASCII, BINARY }
    impl Default for FTPTypes {
        fn default() -> Self { FTPTypes::ASCII }
    }

    #[derive(Default, Debug)]
    pub struct ServerInfo {
        pub mode: FTPModes,
        pub max_connections: i32,
        pub allow_anonymous: bool,
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
        pub user: User,
        pub cwd: String,
        pub connect_mode: FTPModes,
        pub data_type: FTPTypes,
        pub data_ip: String,
        pub data_port: i32,
        pub data_conc: TcpStream,
        pub is_data_up: bool,
        pub is_user_logged: bool, 
        pub is_closing: bool,
        pub is_requesting_login: bool,
        pub is_anon: bool,
        pub placeholder1: String,
    }
    impl Default for ClientConnection {
        fn default() -> Self {
            ClientConnection {
                data_conc: TcpBuilder::new_v4().unwrap()
                    .to_tcp_stream().unwrap(),
                    user: User::default(),
                    cwd: String::default(),
                    connect_mode: FTPModes::default(),
                    data_type: FTPTypes::default(),
                    data_ip: String::default(),
                    data_port: i32::default(),
                    is_data_up: bool::default(),
                    is_user_logged: bool::default(),
                    is_closing: bool::default(),
                    is_requesting_login: bool::default(),
                    is_anon: bool::default(),
                    placeholder1: "".to_string(),
            }
        } 
    }

    #[derive(Default, Debug)]
    pub struct ServerStatus {
        pub is_command_port_open: bool,
        pub active_connections: i32
    }

    #[derive(Debug, Default)]
    pub struct FtpCmd {
        pub _cmd: String,
        pub _args: String,
    }

    #[derive(PartialEq, Debug, Serialize, Deserialize, Copy, Clone)]
    pub enum Rights {
        List, Read, All,
        Nothing
    }
    impl Default for Rights {
        fn default() -> Self { Rights::List }
    }

    #[derive(Default, Debug, Serialize, Deserialize)]
    pub struct User {
        pub username: String,
        pub password: String,
        pub rights: Rights,
    }

    #[derive(Default, Debug, Serialize, Deserialize)]
    pub struct DB {
        pub user: Vec<User>,
    }

    #[derive(Debug)]
    pub enum CmdJob {Add, Remove, List, Clean}
    impl Default for CmdJob {
        fn default() -> Self { CmdJob::List }
    }

    #[derive(Default, Debug)]
    pub struct DBCmd {
        pub job: CmdJob,
        pub user: String,
        pub pass: String,
        pub rights: Rights
    }

    #[derive(Debug, PartialEq)]
    pub enum ClientError {
        Regular(ErrorKind)
    }
    impl Error for ClientError {
        fn description(&self) -> &str {
            match *self {
                ClientError::Regular(ref err) => err.as_str()
            }
        }
    }
    impl fmt::Display for ClientError {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            match *self {
                ClientError::Regular(ref err) => 
                    write!(f, "A client error occured: {:?}", err),
            }
        }
    }

    #[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    pub enum ErrorKind {
        UnrecognizedCmd,
        UnsufficientArgs,
        ProcessCmd,
        NoWait,
    }
    impl ErrorKind {
        fn as_str(&self) -> &str {
            match *self {
                ErrorKind::UnrecognizedCmd => "unrecognized command.",
                ErrorKind::UnsufficientArgs => 
                    "unsufficient arguments.",
                ErrorKind::ProcessCmd => "process cmd.",
                ErrorKind::NoWait => "no wait.",
            }
        }
    }

    #[derive(Debug)]
    pub struct ServerConnection {
        pub connect_mode: FTPModes,
        pub data_conc: TcpStream,
        pub data_type: FTPTypes,
        pub is_connected: bool,
        pub is_data_up: bool,
        pub is_closing: bool,
    }
    impl Default for ServerConnection {
        fn default() -> Self {
            ServerConnection {
                data_conc: TcpBuilder::new_v4().unwrap()
                    .to_tcp_stream().unwrap(),
                    connect_mode: FTPModes::Passive,
                    data_type: FTPTypes::ASCII,
                    is_data_up: bool::default(),
                    is_closing: bool::default(),
                    is_connected: bool::default(),
            }
        }
    }
}
