use std::net::TcpStream;
use crate::defines::defines::*;
use std::io::Write;

pub fn cmd(mut _stream: &mut TcpStream, mut _user: &mut ClientConnection, _cmd: &FtpCmd) ->
Result<(), Box<dyn std::error::Error>> {
    let banner = "Made by altffour.";
    _stream.write(banner.as_bytes())?;
    return Ok(());
}
