pub mod ftp_server {
    use defines::defines::{ServerInfo, ClientInfo, FTPModes, PortRange};
    use std::net::{TcpListener, TcpStream};
    use std::sync::Arc;
    use std::io::{BufReader, BufWriter, Read, Write, BufRead};
    use ftp::*;
    use auth::*;
    
   #[derive(Default, Debug)]
    pub struct ClientConnection {
        pub user: auth::User,
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
        println!("Starting Server with the following settings:");
        println!("Allowed Modes: {:?}", _info.mode);
        println!("Max Connections allowed: {}", _info.max_connections);
        println!("Port Range: {:?}", _info.port_range);
        println!("Anonymous Access: {}", _info.allow_anonymous);
        println!("Log file: {}", _info.log_file);
        println!("Current Working Directory: {:?}", _info.pwd);
        println!("Started Server!");

        let listener = TcpListener::bind("127.0.0.1:21").expect("Couldn't open server, check permissions!");

        // accept connections in parallel.
        for mut stream in listener.incoming() {
            println!("Handling new client...");
            println!("Client number: {}/{}", _state.active_connections+1, _info.max_connections);
            _state.active_connections += 1;

            let client = std::thread::spawn(move || {
                handle_client(&mut stream.unwrap());
            });
        }
    }
 
    fn handle_client(_stream: &mut TcpStream) {
        _stream.set_read_timeout(Some(std::time::Duration::new(120, 0)));
        ftp::sendReply(_stream, &ftp::reply::READY.to_string(), "rftp"); 
        let mut recieved: String = "".to_string();
        let mut reader = BufReader::new(_stream.try_clone().unwrap());

        // Authentication.
        reader.read_line(&mut recieved);

        // Ping-pong communication.
        loop {
            match reader.read_line(&mut recieved) {
                Ok(bytes_read) => {
                    // successfull read.
                }
                Err(e) => {
                    println!("Connection closed: {}", e); 
                    break;
                }
            }
        }
    }   
}
