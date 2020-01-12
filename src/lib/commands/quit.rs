use std::net::TcpStream;
use crate::ftp::*;
use crate::defines::defines::*;

pub fn cmd(mut _stream: &mut TcpStream, mut _user: &mut ClientConnection, _cmd: &FtpCmd) ->
Result<(), Box<dyn std::error::Error>> {
    ftp::send_reply(&mut _stream, &ftp::reply::CLOSING.to_string(), "One off the racks.")?;
    return Ok(());
}



