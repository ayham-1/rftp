pub mod ftp_server {
    use defines::defines::{ServerInfo, ClientInfo, FTPModes, PortRange};
    use std::net::{TcpListener, TcpStream};
   #[derive(Debug)]
    pub struct ClientConnection {
        pub user: String,
        pub pass: String,
        pub connect_mode: FTPModes,
    }
    #[derive(Debug)]
    pub struct ServerStatus {
        pub is_command_port_open: bool,
        pub active_connections: i32
    }

    pub fn start_server(_info: ServerInfo) {
        println!("Starting Server with following settings:");
        println!("Allowed Modes: {:?}", _info.mode);
        println!("Max Connections allowed: {}", _info.max_connections);
        println!("Port Range: {:?}", _info.port_range);
        println!("Anonymous Access: {}", _info.allow_anonymous);
        println!("Log file: {}", _info.log_file);
        println!("Current Working Directory: {:?}", _info.pwd);

        let mut _state = ServerStatus {
            is_command_port_open: false,
            active_connections: 0
        };

        println!("Started Server!");

        let listener = TcpListener::bind("127.0.0.1:21").unwrap();

        // accept connections in parallel.
        for stream in listener.incoming() {
            std::thread::spawn(move || {
                handle_client(stream.unwrap());
            });
        }
    }

    fn handle_client(stream: TcpStream, _info: &ServerInfo, _state: &ServerStatus) -> Result<()> {
        println!("Handling new client...");
        println!("Client number: {}/{}", _state.active_connections+1, _info.max_connections);
        println!("Waiting for username...");

        Ok(());
    }
}
