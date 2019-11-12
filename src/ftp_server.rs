pub mod ftp_server {
    use defines::defines::{ServerInfo, ClientInfo, FTPModes, PortRange};
    use std::net::{TcpListener, TcpStream};
    use std::sync::Arc;
    
   #[derive(Default, Debug)]
    pub struct ClientConnection {
        pub user: String,
        pub pass: String,
        pub connect_mode: FTPModes,
        pub data_connection_up: bool,
    }
    #[derive(Default, Debug)]
    pub struct ServerStatus {
        pub is_command_port_open: bool,
        pub active_connections: i32
    }

    pub fn start_server(_info: ServerInfo) {
        let mut _state = ServerStatus::default();
        println!("Starting Server with following settings:");
        println!("Allowed Modes: {:?}", _info.mode);
        println!("Max Connections allowed: {}", _info.max_connections);
        println!("Port Range: {:?}", _info.port_range);
        println!("Anonymous Access: {}", _info.allow_anonymous);
        println!("Log file: {}", _info.log_file);
        println!("Current Working Directory: {:?}", _info.pwd);
        println!("Started Server!");

        let listener = TcpListener::bind("127.0.0.1:21").unwrap();

        // accept connections in parallel.
        for stream in listener.incoming() {
            println!("Handling new client...");
            println!("Client number: {}/{}", _state.active_connections+1, _info.max_connections);
            _state.active_connections += 1;

            let client = std::thread::spawn(move || {
                handle_client(stream.unwrap());
            });
        }
    }

    fn handle_client(_stream: TcpStream) {
    }
}
