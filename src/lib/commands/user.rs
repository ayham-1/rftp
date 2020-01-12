use std::net::TcpStream;
use crate::ftp::*;
use crate::defines::defines::*;

pub fn cmd(mut _stream: &mut TcpStream, mut _user: &mut ClientConnection, _cmd: &FtpCmd) ->
Result<(), Box<dyn std::error::Error>> {
    // Do pre-checks.

    // Check if user is already logged in.
    if _user.is_user_logged {
        ftp::send_reply(&mut _stream, &ftp::reply::LOGGED_IN.to_string(), "Already Logged in.")?;
        return Ok(());
    }

    // Apply Command.
    _user.user.username = (&_cmd._args).to_string();

    // Check if anonymous.
    if _user.user.username == "anonymous" {
        _user.is_anon = true;
        _user.is_requesting_login = true;
        return Ok(());
    }
    else {
        _user.is_anon = false;
    }

    ftp::send_reply(&mut _stream, &ftp::reply::NEED_PASSWORD.to_string(), &("User ".to_owned() + _user.user.username.as_str() + " needs password."))?;
    return Ok(());
}
