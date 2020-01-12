use std::net::TcpStream;
use crate::ftp::*;
use crate::defines::defines::*;

pub fn cmd(mut _stream: &mut TcpStream, mut _user: &mut ClientConnection, _cmd: &FtpCmd) ->
Result<(), Box<dyn std::error::Error>> {
    // set connect mode.
    _user.connect_mode = FTPModes::Active;
    // get IP.
    let ipmatch = ftp::PORT_IP.captures(&_cmd._args).unwrap();
    let _ip = str::replace(ipmatch.get(0).map_or("".to_string(), |m| m.as_str().to_string()).as_str(), ",", ".");

    // get PORT.
    let portmatch = ftp::PORT_PRT.captures(&_cmd._args).unwrap();
    let _portstr = portmatch.get(0).map_or("".to_string(), |m| m.as_str().to_string());
    let _port0: i32 = ftp::PORT_OCTI0.captures(&_portstr).unwrap().get(0).map_or(0, |m| m.as_str().parse().unwrap());
    let _port1: i32 = ftp::PORT_OCTI1.captures(&_portstr).unwrap().get(0).map_or(0, |m| m.as_str().parse().unwrap());
    let _port: i32 = (_port0*256)+_port1;

    // set IP and PORT.
    _user.data_ip = _ip;
    _user.data_port = _port;

    // reply
    ftp::send_reply(&mut _stream, &ftp::reply::COMMAND_OK.to_string(), &("PORT command successful."))?;
    return Ok(());
}

