use std::net::TcpStream;
use crate::ftp::*;
use crate::defines::defines::*;

pub fn cmd(mut _stream: &mut TcpStream, 
    mut _user: &mut ClientConnection, _cmd: &FtpCmd) ->
Result<(), Box<dyn std::error::Error>> {
    // Check for permissions.
    if _user.user.rights == Rights::Nothing ||
        _user.user.rights == Rights::List ||
        _user.user.rights == Rights::Read {
        ftp::send_reply(&mut _stream, 
            &ftp::reply::REQUESTED_ACTION_NOT_TAKEN.to_string(), 
            "You don't have permission to do that.")?;
        return Ok(());
    }

    match std::fs::remove_file(&ftp::make_path_jailed(&_cmd._args)) {
        Ok(_v) => {
            ftp::send_reply(&mut _stream, 
                &ftp::reply::REQUESTED_FILE_ACTION_OK.to_string(), 
                "File deleted successfully.")?;
        },
        Err(_e) => {
            ftp::send_reply(&mut _stream, 
                &ftp::reply::REQUESTED_ACTION_NOT_TAKEN.to_string(), 
                "Couldn't delete file.")?;
        }
    }

        return Ok(());
}
