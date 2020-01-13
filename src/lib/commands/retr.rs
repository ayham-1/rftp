use std::net::{Shutdown, TcpStream};
use std::io::{Write, Read};
use crate::ftp::*;
use crate::db::*;
use crate::defines::defines::*;
use net2::TcpBuilder;
use std::process::{Command, Stdio};

pub fn cmd(mut _stream: &mut TcpStream, 
    mut _user: &mut ClientConnection, _cmd: &FtpCmd) ->
Result<(), Box<dyn std::error::Error>> {
    // Check for permissions.
    if _user.user.rights == db::Rights::Nothing ||
        _user.user.rights == db::Rights::Read {
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
        _user.data_conc = TcpBuilder::new_v4().unwrap().
            reuse_address(true).unwrap().bind("0.0.0.0:20").unwrap()
            .connect(address.as_str()).unwrap();
    } 

    ftp::send_reply(&mut _stream, 
        &ftp::reply::ABOUT_TO_SEND.to_string(), 
        "Opening Data connection.")?;

    // Read all data.
    if _user.data_type == FTPTypes::ASCII {
        let mut buf = String::new();
        match std::fs::File::open(&ftp::make_path_jailed(&_cmd._args)) {
            Ok(mut _v) => {
                match _v.read_to_string(&mut buf) {
                    Ok(_v) => {},
                    Err(_e) => {
                        ftp::send_reply(&mut _stream, 
                            &ftp::reply::REQUESTED_ACTION_NOT_TAKEN
                            .to_string(), "Could not read file.")?;
                        return Err(Box::new(_e));
                    }
                }
            },
            Err(_e) => {
                ftp::send_reply(&mut _stream, 
                    &ftp::reply::REQUESTED_ACTION_NOT_TAKEN.to_string(),
                    "Could not open file.")?;
                return Err(Box::new(_e));
            }
        }
        // Apply CLRF
        let clrfconv = Command::new("awk")
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .arg(r#"{printf "%s\r\n", $0}"#)
            .spawn().expect("awk command not found.");
        clrfconv.stdin.unwrap().write_all(&buf.as_bytes())?;

        // Send all data.
        let mut result = String::new();
        clrfconv.stdout.unwrap().read_to_string(&mut result)?;
        _user.data_conc.write(result.as_bytes())?;
    }
    else if _user.data_type == FTPTypes::BINARY {
        let mut buf = vec![];
        match std::fs::File::open(&ftp::make_path_jailed(&_cmd._args)) {
            Ok(mut _v) => {
                match _v.read_to_end(&mut buf) {
                    Ok(_v) => {},
                    Err(_e) => {
                        ftp::send_reply(&mut _stream, 
                            &ftp::reply::REQUESTED_ACTION_NOT_TAKEN
                            .to_string(), "Could not read file.")?;
                        return Err(Box::new(_e));
                    }
                }
            },
            Err(_e) => {
                ftp::send_reply(&mut _stream, 
                    &ftp::reply::REQUESTED_ACTION_NOT_TAKEN.to_string(),
                    "Could not open file.")?;
                return Err(Box::new(_e));
            }
        }
        _user.data_conc.write(&buf)?;
    } 

    ftp::send_reply(&mut _stream, 
        &ftp::reply::CLOSING_DATA_CONNECTION.to_string(), 
        "Successfully transferred.")?;
    _user.data_conc.shutdown(Shutdown::Both)?;
    return Ok(());
}


