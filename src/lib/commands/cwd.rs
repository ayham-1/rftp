use std::net::TcpStream;
use crate::ftp::*;
use crate::defines::defines::*;
use std::env;
use std::path::Path;

pub fn cmd(mut _stream: &mut TcpStream, mut _user: &mut ClientConnection, _cmd: &FtpCmd) ->
Result<(), Box<dyn std::error::Error>> {
    let result = env::set_current_dir(Path::new(&ftp::make_path_jailed(&_cmd._args)));
    match result {
        Ok(_v) => {
            _user.cwd = env::current_dir()?.into_os_string().into_string().unwrap();
            ftp::send_reply(&mut _stream, &ftp::reply::REQUESTED_FILE_ACTION_OK.to_string(), "CWD Command Successful.")?;
        },
        Err(_v) => {
            ftp::send_reply(&mut _stream, &ftp::reply::FILE_UNAVAILABLE.to_string(), "CWD Command Failed.")?;
        }
    }
    return Ok(());
}

