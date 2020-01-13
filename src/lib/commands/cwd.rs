use std::net::TcpStream;
use crate::ftp::*;
use crate::defines::defines::*;
use std::env;
use std::path::Path;

pub fn cmd(mut _stream: &mut TcpStream, 
    mut _user: &mut ClientConnection, _cmd: &FtpCmd) ->
Result<(), Box<dyn std::error::Error>> {
    // Check for permissions.
    if _user.user.rights == Rights::Nothing {
        ftp::send_reply(&mut _stream, 
            &ftp::reply::NOT_AVAILABLE.to_string(), 
            "You don't have permission to do that.")?;
        return Ok(());
    }

    let result = env::set_current_dir(Path::new(&
            &_cmd._args));
    match result {
        Ok(_v) => {
            if !ftp::check_current_path_jailness() {
                env::set_current_dir("/var/rftp/")?;
            }
            ftp::send_reply(&mut _stream, 
                &ftp::reply::REQUESTED_FILE_ACTION_OK.to_string(), 
                "CWD Command Successful.")?;
        },
        Err(_v) => {
            ftp::send_reply(&mut _stream, 
                &ftp::reply::FILE_UNAVAILABLE.to_string(), 
                "CWD Command Failed.")?;
        }
    }
    return Ok(());
}
