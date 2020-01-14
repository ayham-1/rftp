use std::net::TcpStream;
use crate::ftp::*;
use crate::defines::defines::*;
use std::env;

pub fn cmd(mut _stream: &mut TcpStream, 
    mut _user: &mut ClientConnection, _cmd: &FtpCmd) ->
Result<(), Box<dyn std::error::Error>> {
    // Get current directory.
    let mut dir = env::current_dir().unwrap().into_os_string()
        .into_string().unwrap();
    dir = ftp::strip_jailness(&dir);
    ftp::send_reply(&mut _stream, 
        &ftp::reply::PATH_CREATED.to_string(), 
        &dir)?;
    return Ok(());
}
