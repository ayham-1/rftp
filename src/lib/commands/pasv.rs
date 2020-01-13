use std::net::{TcpStream, TcpListener};
use crate::ftp::*;
use crate::defines::defines::*;

pub fn cmd(mut _stream: &mut TcpStream, 
    mut _user: &mut ClientConnection, _cmd: &FtpCmd) ->
Result<(), Box<dyn std::error::Error>> {
    // set connection mode.
    _user.connect_mode = FTPModes::Passive;
    // get available port for connection.
    let _open_port = ftp::get_available_port().unwrap();
    let _port_octi0 = _open_port >> 8;
    let _port_octi1 = _open_port - (_port_octi0 * 256);

    // get IP
    let _ip = ftp::get_machine_ip();

    // port reply
    let mut _port: String = String::new();
    _port.push_str("Passive is the way to go. ");
    _port.push_str(&_ip);
    _port.push_str(&_port_octi0.to_string());
    _port.push_str(",");
    _port.push_str(&_port_octi1.to_string());

    ftp::send_reply(&mut _stream, 
        &ftp::reply::PASSIVE_MODE.to_string(), 
        _port.as_str())?;

    let mut _address = String::new();
    _address.push_str("0.0.0.0:");
    _address.push_str(&_open_port.to_string());
    let listener =  TcpListener::bind(&_address).unwrap();
    for stream in listener.incoming() {
        _user.data_conc = stream.unwrap();
        break;
    }

    return Ok(());
}

