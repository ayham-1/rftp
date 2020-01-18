use crate::defines::defines::*;
use std::net::TcpStream;
use crate::ftp::*;

pub fn cmd(mut _stream: &mut TcpStream, _cmd: &str) ->
Result<(), ClientError> {
    if _cmd == "RMD" {
        return Err(ClientError::Regular(ErrorKind::UnsufficientArgs));
    }
    let args = ftp::get_args(&_cmd.to_owned().to_string());
    match ftp::send_client_reply(&mut _stream, "RMD", &args) {
        Ok(_v) => {},
        Err(_e) => {
            return Err(ClientError::Regular(
                    ErrorKind::ProcessCmd));
        }
    }
    return Ok(());
}
