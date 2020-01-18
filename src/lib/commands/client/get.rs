use crate::ftp::*;
use crate::defines::defines::*;
use std::net::{TcpListener, TcpStream, Shutdown};
use std::io::{Write, Read};
use std::fs::OpenOptions;

pub fn cmd(mut _stream: &mut TcpStream, _cmd: &str, 
    mut _server: &mut ServerConnection) -> Result<(), ClientError> {
    if _cmd == "GET" {
        return Err(ClientError::Regular(ErrorKind::UnsufficientArgs));
    }
    // Get file name locally and 
    let local_name;
    let dest_name;
    let split = _cmd.split(" ");
    let vec = split.collect::<Vec<&str>>();
    if vec.len() == 3 {
        dest_name = &vec[1];
        local_name = &vec[2];
    }
    else {
        return Err(ClientError::Regular(ErrorKind::UnsufficientArgs));
    }

    if _server.data_type == FTPTypes::ASCII {
        match ftp::send_client_reply(&mut _stream, "TYPE", "A") {
            Ok(_v) => {
                ftp::print_reply(&_stream).unwrap();
            },
            Err(_e) => {
                return Err(ClientError::Regular(
                        ErrorKind::ProcessCmd));
            }
        } 
    } else if _server.data_type == FTPTypes::BINARY {
        match ftp::send_client_reply(&mut _stream, "TYPE", "I") {
            Ok(_v) => {
                ftp::print_reply(&_stream).unwrap();
            },
            Err(_e) => {
                return Err(ClientError::Regular(
                        ErrorKind::ProcessCmd));
            }
        }
    }

    if _server.connect_mode == FTPModes::Active {
        // Start listening for data connection.
        // Get avaiable PORT
        let _open_port = ftp::get_available_port().unwrap();

        // Calculate PORT OCTIs
        let _port_octi0 = _open_port >> 8; 
        let _port_octi1 = _open_port - (_port_octi0 * 256);
        
        // Make PORT string.
        let _port: String  = 
            _port_octi0.to_string() + "," + &_port_octi1.to_string();

        // Get machine IP.
        let _ip = ftp::get_machine_ip();

        // Make statement.
        let arg = _ip.to_owned() + &_port;

        // Open PORT for listening.
        // Generate acceptable argument.
        let mut tcp_socket = "0.0.0.0".to_string();
        tcp_socket += ":";
        tcp_socket += &_open_port.to_string();
        match ftp::send_client_reply(&mut _stream, "PORT", &arg) {
            Ok(_v) => {
                ftp::print_reply(&_stream).unwrap();
            },
            Err(_e) => {
                return Err(ClientError::Regular(
                        ErrorKind::ProcessCmd));
            }
        }

        match ftp::send_client_reply(&mut _stream, "RETR", &dest_name) {
            Ok(_v) => {},
            Err(_e) => {
                return Err(ClientError::Regular(
                        ErrorKind::ProcessCmd));
            }
        }

        let listener = TcpListener::bind(tcp_socket)
            .expect("Could not open data connection.");
        for stream in listener.incoming() {
            _server.data_conc = stream.unwrap();
            break;
        }
    } else if _server.connect_mode == FTPModes::Passive {
        // Send PASV Command.
        let mut _pasv_arg;
        match ftp::send_client_reply(&mut _stream, "PASV", "") {
            Ok(_v) => {
                _pasv_arg = ftp::get_reply(&_stream);
            },
            Err(_e) => {
                return Err(ClientError::Regular(
                        ErrorKind::ProcessCmd));
            }
        }

        // Get the PORT without the message.
        let split = _pasv_arg.as_str().split(". ");
        let vec = split.collect::<Vec<&str>>();
        if vec.len() == 2 {
            _pasv_arg = vec[1].to_owned().to_string(); 
        }

        let mut ip;
        let port: u32;
        // Split the PORT.
        _pasv_arg = _pasv_arg.replace("\r\n", "");
        let split = _pasv_arg.as_str().split(",");
        let vec = split.collect::<Vec<&str>>();
        if vec.len() == 6 {
            ip = vec[0].to_owned().to_string();
            ip += ".";
            ip += &vec[1].to_owned();
            ip += ".";
            ip += &vec[2].to_owned();
            ip += ".";
            ip += &vec[3].to_owned();
            let oport0: u32 = vec[4].to_owned().parse().unwrap();
            let oport1: u32 = vec[5].to_owned().parse().unwrap();
            port = (oport0*256)+oport1;
        }
        else {
            return Err(ClientError::Regular(
                    ErrorKind::ProcessCmd));
        }

        // Make socket address.
        let _address = ip + ":" + &port.to_string();

        // Connect to server.
        _server.data_conc = TcpStream::connect(&_address).unwrap();


        // Issue append.
        match ftp::send_client_reply(&mut _stream, "RETR", &dest_name) {
            Ok(_v) => {},
            Err(_e) => {
                return Err(ClientError::Regular(
                        ErrorKind::ProcessCmd));
            }
        }
    }
    ftp::print_reply(&_stream).unwrap();

    // Read all data.
    let mut buf = vec![];
    _server.data_conc.read_to_end(&mut buf).unwrap();
    _server.data_conc.shutdown(Shutdown::Both).unwrap();

    // Store all data.
    let mut file = OpenOptions::new().create(true).write(true)
        .append(false).open(local_name).unwrap();
    file.write_all(&buf).unwrap();

    return Ok(());
}
