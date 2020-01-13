use std::net::{Shutdown, TcpStream};
use std::io::{Write, Read};
use crate::ftp::*;
use crate::defines::defines::*;
use net2::TcpBuilder;
use std::process::{Command, Stdio};

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

    if _user.connect_mode == FTPModes::Active {
        // Open data connection.
        let address = &mut _user.data_ip;
        address.push_str(":");
        address.push_str(_user.data_port.to_string().as_str());
        _user.data_conc = TcpBuilder::new_v4().unwrap()
            .reuse_address(true).unwrap().bind("0.0.0.0:20").unwrap()
            .connect(address.as_str()).unwrap();
    } 

    ftp::send_reply(&mut _stream, 
        &ftp::reply::ABOUT_TO_SEND.to_string(), 
        "Opening ASCII Data connection.")?;

    let mut loc = std::env::current_dir().unwrap();
    if _cmd._args != "" {
        loc = loc.join(std::path::Path::new(&_cmd._args));
        if !ftp::check_current_path_jailness() {
            std::env::set_current_dir("/var/rftp/")?;
            loc = std::env::current_dir().unwrap();
        }
    }

    let ls = Command::new("ls")
        .env_clear()
        .arg("-l")
        .arg(loc.into_os_string().into_string().unwrap())
        .output().expect("ls command not found.");
    let clrfconv = Command::new("awk")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .arg(r#"{printf "%s\r\n", $0}"#)
        .spawn().expect("awk command not found.");
    clrfconv.stdin.unwrap().write_all(&ls.stdout)?;
    let mut result = String::new();
    clrfconv.stdout.unwrap().read_to_string(&mut result)?;
    _user.data_conc.write(result.as_bytes())?;
    ftp::send_reply(&mut _stream, 
        &ftp::reply::CLOSING_DATA_CONNECTION.to_string(), 
        "Transfer Complete.")?;
    _user.data_conc.shutdown(Shutdown::Both)?;
    return Ok(());
}
