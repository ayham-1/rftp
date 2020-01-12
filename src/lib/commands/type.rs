use std::net::TcpStream;
use crate::ftp::*;
use crate::defines::defines::*;

pub fn cmd(mut _stream: &mut TcpStream, mut _user: &mut ClientConnection, _cmd: &FtpCmd) ->
Result<(), Box<dyn std::error::Error>> {
    if _cmd._args == "A" {
        _user.data_type = FTPTypes::ASCII;
        ftp::send_reply(&mut _stream, &ftp::reply::COMMAND_OK.to_string(), "Command OK.")?;
    } else if _cmd._args == "I" {
        _user.data_type = FTPTypes::BINARY;
        ftp::send_reply(&mut _stream, &ftp::reply::COMMAND_OK.to_string(), "Command OK.")?;
    }
    else {
        ftp::send_reply(&mut _stream, &ftp::reply::COMMAND_NOT_IMPLEMENTED.to_string(), "This type is not implemented.")?;
    }
    return Ok(());
}
