use crate::defines::defines::*;
use std::net::TcpStream;
use crate::ftp::*;

pub fn cmd(mut _stream: &mut TcpStream, _cmd: &str) ->
Result<(), ClientError> {
    match ftp::send_client_reply(&mut _stream, "CDUP", "") {
        Ok(_v) => {},
        Err(_e) => {
            return Err(ClientError::Regular(
                    ErrorKind::ProcessCmd));
        }
    }
    return Ok(());
}
