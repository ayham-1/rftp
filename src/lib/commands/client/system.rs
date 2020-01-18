use crate::ftp::*;
use crate::defines::defines::*;
use std::net::{TcpStream};

pub fn cmd(mut _stream: &mut TcpStream) ->
Result<(), ClientError> {
    ftp::send_client_reply(_stream, "SYST", "").unwrap(); 
    return Ok(());
}
