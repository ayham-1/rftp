use std::net::TcpStream;
use std::io::Write;
use crate::ftp::*;
use crate::defines::defines::*;
use std::process::Command;

pub fn cmd(mut _stream: &mut TcpStream, 
    mut _user: &mut ClientConnection, _cmd: &FtpCmd) ->
Result<(), Box<dyn std::error::Error>> {
    let stat = Command::new("stat")
        .arg(&ftp::make_path_jailed(&_cmd._args))
        .output().expect("stat command not found.");
    _stream.write(&stat.stdout)?;
    ftp::send_reply(&mut _stream, &ftp::reply::SYSTEM.to_string(), 
        "File info sent.")?;
    return Ok(());
}
