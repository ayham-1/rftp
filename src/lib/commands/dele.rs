use std::net::TcpStream;
use crate::ftp::*;
use crate::defines::defines::*;

pub fn cmd(mut _stream: &mut TcpStream, mut _user: &mut ClientConnection, _cmd: &FtpCmd) ->
Result<(), Box<dyn std::error::Error>> {
    std::fs::remove_file(&ftp::make_path_jailed(&_cmd._args))?;
    ftp::send_reply(&mut _stream, &ftp::reply::REQUESTED_FILE_ACTION_OK.to_string(), "File deleted successfully.")?;
    return Ok(());
}
