use std::net::TcpStream;
use crate::ftp::*;
use crate::db::*;
use crate::defines::defines::*;

pub fn cmd(mut _stream: &mut TcpStream, 
    mut _user: &mut ClientConnection, _cmd: &FtpCmd) ->
Result<(), Box<dyn std::error::Error>> {
    // Check for permissions.
    if _user.user.rights == db::Rights::Nothing ||
        _user.user.rights == db::Rights::List ||
        _user.user.rights == db::Rights::Read {
        ftp::send_reply(&mut _stream, 
            &ftp::reply::NOT_AVAILABLE.to_string(), 
            "You don't have permission to do that.")?;       
        return Ok(());
    }

    // Set placeholder name.
    _user.placeholder1 = _cmd._args.to_owned();
    ftp::send_reply(&mut _stream, 
        &ftp::reply::REQUEST_FILE_PENDING.to_string(), 
        "Placeholder set.")?;
    return Ok(());
}


