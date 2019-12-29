pub mod ftp_server {
    use defines::defines::{ServerInfo, FTPModes};
    use std::net::{TcpListener, TcpStream};
    use std::sync::{Arc, Mutex};
    use std::thread;
    use std::io::{BufReader, BufRead};
    use ftp::*;
    use db::*;
    use serverPI::*;
    
   #[derive(Default, Debug)]
    pub struct ClientConnection {
        pub user: db::User,
        pub connect_mode: FTPModes,
        pub is_data_up: bool,
        pub is_user_logged: bool, 
        pub is_closing: bool,
        pub is_requesting_login: bool,
        pub is_anon: bool,
    }
    #[derive(Default, Debug)]
    pub struct ServerStatus {
        pub is_command_port_open: bool,
        pub active_connections: i32
    }

    pub fn start_server(_info: ServerInfo) {
        println!("Initializing Authorization Database...");
        let mut db = Arc::new(Mutex::new(db::loadDB()));

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

            let _db = Arc::clone(&db);
            let mut clientName: String = "Client#".to_string();
            clientName.push_str(&(_state.active_connections.to_string()));
            let builder = thread::Builder::new().name(clientName);
            if _info.allow_anonymous {
                let client = builder.spawn(move || {
                    handle_client(&mut stream.unwrap(), _db, true);
                });
            } else {
                let client = builder.spawn(move || {
                    handle_client(&mut stream.unwrap(), _db, false);
                });
            }
        }
    }
 
    fn handle_client(mut _stream: &mut TcpStream, _db: std::sync::Arc<Mutex<db::DB>>, anon: bool) {
        let mut client: ClientConnection = ClientConnection::default();
        client.is_closing = false;

        _stream.set_read_timeout(Some(std::time::Duration::new(120, 0)));
        ftp::sendReply(_stream, &ftp::reply::READY.to_string(), "rftp"); 
        let mut recieved: String  = "".to_string();
        let mut reader = BufReader::new(_stream.try_clone().unwrap());

        // Authentication.
        reader.read_line(&mut recieved);
        serverPI::applyCMD(&mut _stream, &mut client, &mut (serverPI::parseftpCMD((&recieved).to_string())));
        if client.is_requesting_login {
            logginUser(&mut _stream, &mut client, &mut _db.lock().unwrap(), anon);
        }
        else {
            recieved = "".to_string();
            reader.read_line(&mut recieved);
            serverPI::applyCMD(&mut _stream, &mut client, &mut (serverPI::parseftpCMD((&recieved).to_string())));
            if client.is_requesting_login {
                logginUser(&mut _stream, &mut client, &mut _db.lock().unwrap(), anon);
            }
        }

        // Ping-pong communication.
        loop {
            recieved = "".to_string();
            if client.is_closing {
                println!("Connection Closed!"); 
                return;
            }
            match reader.read_line(&mut recieved) {
                Ok(bytes_read) => {
                    if bytes_read == 0 {
                        println!("Connection Closed!"); 
                        return;
                    }
                    // successful read.
                    let mut cmd = serverPI::parseftpCMD((&recieved).to_string());
                    serverPI::applyCMD(&mut _stream, &mut client, &mut cmd);
                }
                Err(e) => {
                    println!("Connection closed: {}", e); 
                    return;
                }
            }
        }
    }

    pub fn logginUser(mut _stream: &mut TcpStream, mut client: &mut ClientConnection, _db: &db::DB, anon: bool) {
        // Pre-checks.

        // Check if it is anonymous loggin.
        if client.is_anon == true {
            println!("Client logged in as anonymous!");
            if anon {
                ftp::sendReply(&mut _stream, &ftp::reply::LOGGED_IN.to_string(), &("User logged in as anonymous."));
                client.is_user_logged = true;
            }
            else {
                ftp::sendReply(&mut _stream, &ftp::reply::NOT_LOGGED_IN.to_string(), &("Anonymous is disabled on this server."));
                client.is_user_logged = false;
                client.is_closing = true;
            }
            return;
        }
        
        // Check if credientials are present.
        if client.user.username == "" && client.user.password == "" {
            ftp::sendReply(&mut _stream, &ftp::reply::BAD_ARGUMENTS.to_string(), "Credientails are empty.");
            return;
        }

        // Try to loggin user.
        for i in _db.user.iter() {
            if client.user.username == i.username && client.user.password == i.password {
                client.user.rights = i.rights;
                client.is_user_logged = true;
                println!("Client logged in!");
                let mut result: String = "User ".to_string();
                result.push_str(&client.user.username);
                result.push_str(&(" logged in.".to_string()));
                ftp::sendReply(&mut _stream, &ftp::reply::LOGGED_IN.to_string(), &result);
                return;
            }
        }

        println!("Unsuccessful loggin attempt.");
        ftp::sendReply(&mut _stream, &ftp::reply::CLOSING.to_string(), "Bad credientails.");
        client.is_closing = true;
    }
}
