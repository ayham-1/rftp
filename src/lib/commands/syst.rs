use std::net::TcpStream;
use crate::ftp::*;
use crate::defines::defines::*;

pub fn cmd(mut _stream: &mut TcpStream, 
    mut _user: &mut ClientConnection, _cmd: &FtpCmd) ->
Result<(), Box<dyn std::error::Error>> {
    ftp::send_reply(&mut _stream, &ftp::reply::NAME.to_string(), 
        "UNIX Type: L8")?;
    return Ok(());
}
