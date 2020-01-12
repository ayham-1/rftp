use std::net::TcpStream;
use std::io::{Write, Read};
use crate::ftp::*;
use crate::defines::defines::*;
use std::fs::OpenOptions;
use net2::TcpBuilder;

pub fn cmd(mut _stream: &mut TcpStream, mut _user: &mut ClientConnection, _cmd: &FtpCmd) ->
Result<(), Box<dyn std::error::Error>> {
    // Open Data connection.
    if _user.connect_mode == FTPModes::Active {
        // Open data connection.
        let address = &mut _user.data_ip;
        address.push_str(":");
        address.push_str(_user.data_port.to_string().as_str());
        _user.data_conc = TcpBuilder::new_v4().unwrap().reuse_address(true).unwrap().bind("0.0.0.0:20").unwrap().connect(address.as_str()).unwrap();
    }

    ftp::send_reply(&mut _stream, &ftp::reply::ABOUT_TO_SEND.to_string(), "Open data channel for file upload.")?;

    // Read all data.
    let mut buf = vec![];
    _user.data_conc.read_to_end(&mut buf)?;

    // Store all data.
    let mut file = OpenOptions::new().create(true).write(true).append(false).open(&ftp::make_path_jailed(&_cmd._args))?;
    file.write_all(&buf)?;

    ftp::send_reply(&mut _stream, &ftp::reply::CLOSING_DATA_CONNECTION.to_string(), "Successfully transferred.")?;
    return Ok(());
}

