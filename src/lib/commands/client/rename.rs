use crate::defines::defines::*;
use std::net::TcpStream;
use crate::ftp::*;

pub fn cmd(mut _stream: &mut TcpStream, _cmd: &str) ->
Result<(), ClientError> {
    if _cmd == "RENAME" {
        return Err(ClientError::Regular(ErrorKind::UnsufficientArgs));
    }
    let rnfr;
    let rnto;
    let split = _cmd.split(" ");
    let vec = split.collect::<Vec<&str>>();
    if vec.len() == 3 {
        rnfr = &vec[1];
        rnto = &vec[2];
    }
    else {
        return Err(ClientError::Regular(ErrorKind::UnsufficientArgs));
    }
    
    match ftp::send_client_reply(&mut _stream, "RNFR", rnfr) {
        Ok(_v) => {
            match ftp::send_client_reply(&mut _stream, "RNTO", rnto) {
                Ok(_v) => {},
                Err(_e) => {
                    return Err(ClientError::Regular(
                            ErrorKind::ProcessCmd));
                }
            }
        },
        Err(_e) => {
            return Err(ClientError::Regular(
                    ErrorKind::ProcessCmd));
        }
    }
    return Ok(());
}
