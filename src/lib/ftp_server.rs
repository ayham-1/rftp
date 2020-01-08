pub mod ftp_server {
    use crate::defines::defines::*;
    use std::net::{TcpListener, TcpStream};
    use std::sync::{Arc, Mutex};
    use std::thread;
    use std::io::{BufReader, BufRead};
    use crate::ftp::*;
    use crate::db::*;
    use crate::server_pi::*;
   
    pub fn start_server(_info: ServerInfo) -> Result<(), Box<dyn std::error::Error>>{
        println!("Initializing Authorization Database...");
        let db = Arc::new(Mutex::new(db::load_db()?));

        let mut _state = ServerStatus::default();
        println!("Starting Server with the following settings:");
        println!("Allowed Modes: {:?}", _info.mode);
        println!("Max Connections allowed: {}", _info.max_connections);
        println!("Port Range: {:?}", _info.port_range);
        println!("Anonymous Access: {}", _info.allow_anonymous);
        println!("Log file: {}", _info.log_file);
        println!("Current Working Directory: {:?}", _info.pwd);
        println!("Started Server!");

        let _linfo = Arc::new(Mutex::new(_info));
        let _lstate = Arc::new(Mutex::new(_state));

        let listener = TcpListener::bind("0.0.0.0:21").expect("Couldn't open server, run with sudo!");

        // accept connections in parallel.
        for stream in listener.incoming() {
            let _info = Arc::clone(&_linfo);
            let _lstate = Arc::clone(&_lstate);
            let _db = Arc::clone(&db);

            println!("Handling new client...");
            println!("Client number: {}/{}", _lstate.lock().unwrap().active_connections+1, _info.lock().unwrap().max_connections);
            _lstate.lock().unwrap().active_connections += 1;

            let mut client_name: String = "Client#".to_string();
            client_name.push_str(&(_lstate.lock().unwrap().active_connections.to_string()));
            let builder = thread::Builder::new().name(client_name);
                builder.spawn(move || {
                    match handle_client(&mut stream.unwrap(), _db, _info.lock().unwrap().allow_anonymous) {
                        Ok(_v) => {},
                        Err(_e) => {
                            println!("Error handling {}", std::thread::current().name().unwrap());
                        }
                    }
                    println!("Client#{} got enough of their misery", _lstate.lock().unwrap().active_connections);
                    _lstate.lock().unwrap().active_connections -= 1;
                })?;
        }
        Ok(())
    }
 
    fn handle_client(mut _stream: &mut TcpStream, _db: std::sync::Arc<Mutex<db::DB>>, anon: bool) ->
        Result<(), Box<dyn std::error::Error>> {
        let mut client: ClientConnection = ClientConnection::default();
        client.is_closing = false;

        _stream.set_read_timeout(Some(std::time::Duration::new(120, 0)))?;
        ftp::send_reply(_stream, &ftp::reply::READY.to_string(), "rftp")?;
        let mut recieved: String  = "".to_string();
        let mut reader = BufReader::new(_stream.try_clone()?);

        // Authentication.
        reader.read_line(&mut recieved)?;
        server_pi::apply_cmd(&mut _stream, &mut client, &mut (server_pi::parseftp_cmd((&recieved).to_string())))?;
        if client.is_requesting_login {
            loggin_user(&mut _stream, &mut client, &mut _db.lock().unwrap(), anon)?;
        }
        else {
            recieved = "".to_string();
            reader.read_line(&mut recieved)?;
            server_pi::apply_cmd(&mut _stream, &mut client, &mut (server_pi::parseftp_cmd((&recieved).to_string())))?;
            if client.is_requesting_login {
                loggin_user(&mut _stream, &mut client, &mut _db.lock().unwrap(), anon)?;
            }
        }

        // Ping-pong communication.
        loop {
            recieved = "".to_string();
            if client.is_closing {
                println!("Connection Closed!"); 
                return Ok(())
            }
            match reader.read_line(&mut recieved) {
                Ok(bytes_read) => {
                    if bytes_read == 0 {
                        println!("Connection Closed!"); 
                        return Ok(())
                    }
                    // successful read.
                    let mut cmd = server_pi::parseftp_cmd((&recieved).to_string());
                    server_pi::apply_cmd(&mut _stream, &mut client, &mut cmd)?;
                }
                Err(e) => {
                    println!("Connection closed: {}", e); 
                    return Ok(())
                }
            }
        }
    }

    pub fn loggin_user(mut _stream: &mut TcpStream, mut client: &mut ClientConnection, 
        _db: &db::DB, anon: bool) -> 
        Result<(), Box<dyn std::error::Error>> {
        // Pre-checks.

        // Check if it is anonymous loggin.
        if client.is_anon == true {
            println!("Client logged in as anonymous!");
            if anon {
                ftp::send_reply(&mut _stream, &ftp::reply::LOGGED_IN.to_string(), 
                    &("User logged in as anonymous."))?;
                client.is_user_logged = true;
            }
            else {
                ftp::send_reply(&mut _stream, &ftp::reply::NOT_LOGGED_IN.to_string(), 
                    &("Anonymous is disabled on this server."))?;
                client.is_user_logged = false;
                client.is_closing = true;
            }
            return Ok(());
        }
        
        // Check if credientials are present.
        if client.user.username == "" && client.user.password == "" {
            ftp::send_reply(&mut _stream, &ftp::reply::BAD_ARGUMENTS.to_string(), "Credientails are empty.")?;
            return Ok(());
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
                ftp::send_reply(&mut _stream, &ftp::reply::LOGGED_IN.to_string(), &result)?;
                return Ok(());
            }
        }

        println!("Unsuccessful loggin attempt.");
        ftp::send_reply(&mut _stream, &ftp::reply::CLOSING.to_string(), "Bad credientails.")?;
        client.is_closing = true;
        return Ok(());
    }
}
