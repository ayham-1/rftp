use std::net::TcpStream;
use crate::ftp::*;
use crate::db::*;
use crate::defines::defines::*;
use std::env;
use std::path::Path;

pub fn cmd(mut _stream: &mut TcpStream, 
    mut _user: &mut ClientConnection, _cmd: &FtpCmd) ->
Result<(), Box<dyn std::error::Error>> {
    // Check for permissions.
    if _user.user.rights == db::Rights::Nothing {
        ftp::send_reply(&mut _stream, 
            &ftp::reply::NOT_AVAILABLE.to_string(), 
            "You don't have permissiont to do that.")?;       
        return Ok(());
    }

    let result = env::set_current_dir(Path::new(&
            ftp::make_path_jailed("..")));
    match result {
        Ok(_v) => {
            _user.cwd = env::current_dir()?.into_os_string()
                .into_string().unwrap();
            ftp::send_reply(&mut _stream, 
                &ftp::reply::REQUESTED_FILE_ACTION_OK.to_string(), 
                "CDUP Command Successful.")?;
        },
        Err(_v) => {
            ftp::send_reply(&mut _stream, 
                &ftp::reply::FILE_UNAVAILABLE.to_string(), 
                "CDUP Command Failed.")?;
        }
    }
    return Ok(());
}
