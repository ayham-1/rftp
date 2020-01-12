use std::net::TcpStream;
use crate::ftp::*;
use crate::defines::defines::*;

pub fn cmd(mut _stream: &mut TcpStream, mut _user: &mut ClientConnection, _cmd: &FtpCmd) ->
Result<(), Box<dyn std::error::Error>> {
    // Rename file.
    match std::fs::rename(&ftp::make_path_jailed(&_user.placeholder1), &ftp::make_path_jailed(&_cmd._args)) {
        Ok(_v) => {
            ftp::send_reply(&mut _stream, &ftp::reply::REQUESTED_FILE_ACTION_OK.to_string(), "Renamed file.")?;
        },
        Err(_e) => {
            ftp::send_reply(&mut _stream, &ftp::reply::REQUESTED_ACTION_NOT_TAKEN.to_string(), "Faied to rename file.")?;
        }
    }
    return Ok(());
}

