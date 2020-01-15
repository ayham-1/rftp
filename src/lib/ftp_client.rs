pub mod ftp_client {
    use crate::ftp::*;
    use crate::defines::defines::*;
    use crate::client_pi::*;

    use std::error::Error;
    use std::net::{TcpStream, ToSocketAddrs, Shutdown};
    use std::io::{self};
    use std::io::prelude::*;
    use log::{info, error, warn};
    use net2::TcpBuilder;

    pub fn start_client(_info: ClientInfo) -> Result<(), 
    Box<dyn Error>> { 
        // Figure out the IP.
        let mut _address_iter = _info.server_name.as_str()
            .to_socket_addrs()?;

        // Connect to server.
        let mut _address = "".to_string();
        let mut _stream = TcpBuilder::new_v4()?.to_tcp_stream()?;
        let mut _address_iter_val = _address_iter.next();
        let mut _stream_iter = 
            TcpStream::connect(_address_iter_val.unwrap());
        while _address_iter_val.is_some() && _stream_iter.is_err() {
            _address = _address_iter_val.unwrap().to_string();
            _stream_iter = 
                TcpStream::connect(_address_iter_val.unwrap());
            _address_iter_val = _address_iter.next();
        }
        if _stream_iter.is_err() {
            error!("Could not find an available Ipv4 address.");
            Err("Error connecting.")?;
        } else {
            _stream = _stream_iter.unwrap();
            info!("Connected to {}", _address);
        }
        
        // Start recieving.
        ftp::print_reply(&_stream)?;

        // Authentication.
        let mut name: String = "".to_string();
        let mut pass: String = "".to_string();
        
        print!("Name: ");
        io::stdout().flush()?;
        io::stdin().read_line(&mut name)?;
        ftp::send_client_reply(&mut _stream, "USER", &name)?;
        ftp::print_reply(&_stream)?;

        print!("Password: ");
        io::stdout().flush()?;
        io::stdin().read_line(&mut pass)?;
        ftp::send_client_reply(&mut _stream, "PASS", &pass)?;
        pass.clear(); // Somewhat prevent memory attacks.
        ftp::print_reply(&_stream)?;

        loop {
            let mut received = "".to_string();
            print!("> ");
            io::stdout().flush()?;
            io::stdin().read_line(&mut received)?; 

            match client_pi::send_cmd(&mut _stream, &received) {
                Ok(_v) => {
                    ftp::print_reply(&_stream)?;
                },
                Err(_e) => {
                    warn!("{}", _e);
                }
            }
        };
        _stream.shutdown(Shutdown::Both)?;        

        return Ok(());
    }
}
