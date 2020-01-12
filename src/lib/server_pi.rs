pub mod server_pi {
    use std::net::{Shutdown, TcpStream, TcpListener};
    use crate::ftp::*;
    use crate::defines::defines::*;
    use std::process::{Command, Stdio};
    use std::io::{Write, Read};
    use net2::TcpBuilder;
    use std::path::Path;
    use std::env;
    use std::str;
    use std::fs::OpenOptions;

    #[derive(Debug, Default)]
    pub struct FtpCmd {
        pub _cmd: String,
        pub _args: String,
    }

    pub fn parseftp_cmd(_recieved: String) -> FtpCmd {
        let mut result: FtpCmd = FtpCmd::default();
        if _recieved == "" { return result; }

        result._cmd = ftp::get_command(&_recieved);
        result._args = ftp::get_args(&_recieved);
        return result;
    }

    pub fn apply_cmd(mut _stream: &mut TcpStream, mut _user: &mut ClientConnection, _cmd: &FtpCmd) ->
        Result<(), Box<dyn std::error::Error>> {
            let cmd = _cmd._cmd.as_str();
            match cmd {
                "USER" => process_user_cmd(&mut _stream, &mut _user, &_cmd)?,
                "PASS" => process_pass_cmd(&mut _stream, &mut _user, &_cmd)?,
                "SYST" => process_syst_cmd(&mut _stream, &mut _user, &_cmd)?,
                "QUIT" => process_quit_cmd(&mut _stream, &mut _user, &_cmd)?,
                "PORT" => process_port_cmd(&mut _stream, &mut _user, &_cmd)?,
                "PASV" => process_pasv_cmd(&mut _stream, &mut _user, &_cmd)?,
                "TYPE" => process_type_cmd(&mut _stream, &mut _user, &_cmd)?,
                "LIST" => process_list_cmd(&mut _stream, &mut _user, &_cmd)?,
                "HELP" => process_help_cmd(&mut _stream, &mut _user, &_cmd)?,
                "NOOP" => process_noop_cmd(&mut _stream, &mut _user, &_cmd)?,
                "CWD" => process_cwd_cmd(&mut _stream, &mut _user, &_cmd)?,
                "MODE" => process_mode_cmd(&mut _stream, &mut _user, &_cmd)?,
                "ACCT" => process_acct_cmd(&mut _stream, &mut _user, &_cmd)?,
                "STRU" => process_stru_cmd(&mut _stream, &mut _user, &_cmd)?,
                "STOR" => process_stor_cmd(&mut _stream, &mut _user, &_cmd)?,
                "ABOR" => process_abor_cmd(&mut _stream, &mut _user, &_cmd)?,
                "ALLO" => process_allo_cmd(&mut _stream, &mut _user, &_cmd)?,
                "APPE" => process_appe_cmd(&mut _stream, &mut _user, &_cmd)?,
                "DELE" => process_dele_cmd(&mut _stream, &mut _user, &_cmd)?,
                "NLST" => process_nlst_cmd(&mut _stream, &mut _user, &_cmd)?,
                "REIN" => process_rein_cmd(&mut _stream, &mut _user, &_cmd)?,
                _ => { 
                    ftp::send_reply(&mut _stream, &ftp::reply::COMMAND_NOT_IMPLEMENTED.to_string(), "Command Not Implemented.")?;
                }
            }
            return Ok(());
    }

    pub fn process_rein_cmd(mut _stream: &mut TcpStream, mut _user: &mut ClientConnection, _cmd: &FtpCmd) ->
        Result<(), Box<dyn std::error::Error>> {
            ftp::send_reply(&mut _stream, &ftp::reply::COMMAND_NOT_IMPLEMENTED.to_string(), "Closing connection, command not implemented.")?;
            _user.is_closing = true;
            return Ok(());
    }

    pub fn process_nlst_cmd(mut _stream: &mut TcpStream, mut _user: &mut ClientConnection, _cmd: &FtpCmd) ->
        Result<(), Box<dyn std::error::Error>> {
            if _user.connect_mode == FTPModes::Active {
                // Open data connection.
                let address = &mut _user.data_ip;
                address.push_str(":");
                address.push_str(_user.data_port.to_string().as_str());
                _user.data_conc = TcpBuilder::new_v4().unwrap().
                    reuse_address(true).unwrap().bind("0.0.0.0:20").unwrap().connect(address.as_str()).unwrap();
            } 

            ftp::send_reply(&mut _stream, &ftp::reply::ABOUT_TO_SEND.to_string(), "Opening ASCII Data connection.")?;
            let ls = Command::new("ls")
                .env_clear()
                .arg(&ftp::make_path_jailed(&_cmd._args))
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
            ftp::send_reply(&mut _stream, &ftp::reply::CLOSING_DATA_CONNECTION.to_string(), "Transfer Complete.")?;
            _user.data_conc.shutdown(Shutdown::Both)?;
            return Ok(());
    }

    pub fn process_dele_cmd(mut _stream: &mut TcpStream, mut _user: &mut ClientConnection, _cmd: &FtpCmd) ->
        Result<(), Box<dyn std::error::Error>> {
            std::fs::remove_file(&ftp::make_path_jailed(&_cmd._args))?;
            ftp::send_reply(&mut _stream, &ftp::reply::REQUESTED_FILE_ACTION_OK.to_string(), "File deleted successfully.")?;
            return Ok(());
    }

    pub fn process_appe_cmd(mut _stream: &mut TcpStream, mut _user: &mut ClientConnection, _cmd: &FtpCmd) ->
        Result<(), Box<dyn std::error::Error>> {
            // Open Data connection.
            if _user.connect_mode == FTPModes::Active {
                // Open data connection.
                let address = &mut _user.data_ip;
                address.push_str(":");
                address.push_str(_user.data_port.to_string().as_str());
                _user.data_conc = TcpBuilder::new_v4().unwrap().reuse_address(true).unwrap().bind("0.0.0.0:20").unwrap().connect(address.as_str()).unwrap();
            }

            ftp::send_reply(&mut _stream, &ftp::reply::ABOUT_TO_SEND.to_string(), "Open data channel for file upload.")?;

            // Read all data.
            let mut buf = vec![];
            _user.data_conc.read_to_end(&mut buf)?;

            // Store all data.
            let mut file = OpenOptions::new().create(true).write(true).append(true).open(&ftp::make_path_jailed(&_cmd._args))?;
            file.write_all(&buf)?;

            ftp::send_reply(&mut _stream, &ftp::reply::CLOSING_DATA_CONNECTION.to_string(), "Successfully transferred.")?;
            return Ok(());
    }

    pub fn process_allo_cmd(mut _stream: &mut TcpStream, mut _user: &mut ClientConnection, _cmd: &FtpCmd) ->
        Result<(), Box<dyn std::error::Error>> {
            ftp::send_reply(&mut _stream, &ftp::reply::COMMAND_NOT_IMPLEMENTED.to_string(), "Just send the data already.")?;
            return Ok(());
    }

    pub fn process_abor_cmd(mut _stream: &mut TcpStream, mut _user: &mut ClientConnection, _cmd: &FtpCmd) ->
        Result<(), Box<dyn std::error::Error>> {
            ftp::send_reply(&mut _stream, &ftp::reply::COMMAND_NOT_IMPLEMENTED.to_string(), "Serial data transmittion is currently suppported.")?;
            return Ok(());
    }

    pub fn process_stor_cmd(mut _stream: &mut TcpStream, mut _user: &mut ClientConnection, _cmd: &FtpCmd) ->
        Result<(), Box<dyn std::error::Error>> {
            // Open Data connection.
            if _user.connect_mode == FTPModes::Active {
                // Open data connection.
                let address = &mut _user.data_ip;
                address.push_str(":");
                address.push_str(_user.data_port.to_string().as_str());
                _user.data_conc = TcpBuilder::new_v4().unwrap().reuse_address(true).unwrap().bind("0.0.0.0:20").unwrap().connect(address.as_str()).unwrap();
            }

            ftp::send_reply(&mut _stream, &ftp::reply::ABOUT_TO_SEND.to_string(), "Open data channel for file upload.")?;

            // Read all data.
            let mut buf = vec![];
            _user.data_conc.read_to_end(&mut buf)?;

            // Store all data.
            let mut file = OpenOptions::new().create(true).write(true).append(false).open(&ftp::make_path_jailed(&_cmd._args))?;
            file.write_all(&buf)?;

            ftp::send_reply(&mut _stream, &ftp::reply::CLOSING_DATA_CONNECTION.to_string(), "Successfully transferred.")?;

            return Ok(());
    }

    pub fn process_stru_cmd(mut _stream: &mut TcpStream, mut _user: &mut ClientConnection, _cmd: &FtpCmd) ->
        Result<(), Box<dyn std::error::Error>> {
            ftp::send_reply(&mut _stream, &ftp::reply::COMMAND_NOT_IMPLEMENTED.to_string(), "Only file mode is supported.")?;
            return Ok(());
    }

    pub fn process_acct_cmd(mut _stream: &mut TcpStream, mut _user: &mut ClientConnection, _cmd: &FtpCmd) ->
        Result<(), Box<dyn std::error::Error>> {
            ftp::send_reply(&mut _stream, &ftp::reply::COMMAND_NOT_IMPLEMENTED.to_string(), "Accounts are not Implemented.")?;
            return Ok(());
    }

    pub fn process_mode_cmd(mut _stream: &mut TcpStream, mut _user: &mut ClientConnection, _cmd: &FtpCmd) ->
        Result<(), Box<dyn std::error::Error>> {

            ftp::send_reply(&mut _stream, &ftp::reply::COMMAND_NOT_IMPLEMENTED.to_string(), "Only stream mode is supported.")?;
            return Ok(());
    }

    pub fn process_cwd_cmd(mut _stream: &mut TcpStream, mut _user: &mut ClientConnection, _cmd: &FtpCmd) ->
        Result<(), Box<dyn std::error::Error>> {
            match env::set_current_dir(Path::new(&ftp::make_path_jailed(&_cmd._args))) {
                Ok(_v) => {
                    ftp::send_reply(&mut _stream, &ftp::reply::REQUESTED_FILE_ACTION_OK.to_string(), "CWD Command Successful.")?;
                },
                Err(_v) => {
                    ftp::send_reply(&mut _stream, &ftp::reply::FILE_UNAVAILABLE.to_string(), "CWD Command Failed.")?;
                }
            }
            return Ok(());
    }

    pub fn process_noop_cmd(mut _stream: &mut TcpStream, mut _user: &mut ClientConnection, _cmd: &FtpCmd) ->
        Result<(), Box<dyn std::error::Error>> {
            ftp::send_reply(&mut _stream, &ftp::reply::COMMAND_OK.to_string(), "Stop bothering me.")?;
            return Ok(());
    }

    pub fn process_pasv_cmd(mut _stream: &mut TcpStream, mut _user: &mut ClientConnection, _cmd: &FtpCmd) ->
        Result<(), Box<dyn std::error::Error>> {
            // set connection mode.
            _user.connect_mode = FTPModes::Passive;
            // get available port for connection.
            let _open_port = ftp::get_available_port().unwrap();
            let _port_octi0 = _open_port >> 8;
            let _port_octi1 = _open_port - (_port_octi0 * 256);

            // get IP
            let _ip = ftp::get_machine_ip();

            // port reply
            let mut _port: String = String::new();
            _port.push_str("Passive is the way to go. ");
            _port.push_str(&_ip);
            _port.push_str(&_port_octi0.to_string());
            _port.push_str(",");
            _port.push_str(&_port_octi1.to_string());

            ftp::send_reply(&mut _stream, &ftp::reply::PASSIVE_MODE.to_string(), _port.as_str())?;

            let mut _address = String::new();
            _address.push_str("0.0.0.0:");
            _address.push_str(&_open_port.to_string());
            let listener =  TcpListener::bind(&_address).unwrap();
            for stream in listener.incoming() {
                _user.data_conc = stream.unwrap();
                break;
            }

            return Ok(());
    }

    pub fn process_help_cmd(mut _stream: &mut TcpStream, mut _user: &mut ClientConnection, _cmd: &FtpCmd) ->
        Result<(), Box<dyn std::error::Error>> {
            let banner = "Made by altffour.";
            _stream.write(banner.as_bytes())?;
            return Ok(());
    }

    pub fn process_list_cmd(mut _stream: &mut TcpStream, mut _user: &mut ClientConnection, _cmd: &FtpCmd) ->
        Result<(), Box<dyn std::error::Error>> {
            if _user.connect_mode == FTPModes::Active {
                // Open data connection.
                let address = &mut _user.data_ip;
                address.push_str(":");
                address.push_str(_user.data_port.to_string().as_str());
                _user.data_conc = TcpBuilder::new_v4().unwrap().reuse_address(true).unwrap().bind("0.0.0.0:20").unwrap().connect(address.as_str()).unwrap();
            } 

            ftp::send_reply(&mut _stream, &ftp::reply::ABOUT_TO_SEND.to_string(), "Opening ASCII Data connection.")?;
            let ls = Command::new("ls")
                .env_clear()
                .arg("-l")
                .arg(&ftp::make_path_jailed(&_cmd._args))
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
            ftp::send_reply(&mut _stream, &ftp::reply::CLOSING_DATA_CONNECTION.to_string(), "Transfer Complete.")?;
            _user.data_conc.shutdown(Shutdown::Both)?;
            return Ok(());
        }

    pub fn process_type_cmd(mut _stream: &mut TcpStream, mut _user: &mut ClientConnection, _cmd: &FtpCmd) ->
        Result<(), Box<dyn std::error::Error>> {
            if _cmd._args == "A" {
                _user.data_type = FTPTypes::ASCII;
                ftp::send_reply(&mut _stream, &ftp::reply::COMMAND_OK.to_string(), "Command OK.")?;
            } else if _cmd._args == "I" {
                _user.data_type = FTPTypes::BINARY;
                ftp::send_reply(&mut _stream, &ftp::reply::COMMAND_OK.to_string(), "Command OK.")?;
            }
            else {
                ftp::send_reply(&mut _stream, &ftp::reply::COMMAND_NOT_IMPLEMENTED.to_string(), "This type is not implemented.")?;
            }
            return Ok(());
        }

    pub fn process_port_cmd(mut _stream: &mut TcpStream, mut _user: &mut ClientConnection, _cmd: &FtpCmd) ->
        Result<(), Box<dyn std::error::Error>> {
            // set connect mode.
            _user.connect_mode = FTPModes::Active;
            // get IP.
            let ipmatch = ftp::PORT_IP.captures(&_cmd._args).unwrap();
            let _ip = str::replace(ipmatch.get(0).map_or("".to_string(), |m| m.as_str().to_string()).as_str(), ",", ".");

            // get PORT.
            let portmatch = ftp::PORT_PRT.captures(&_cmd._args).unwrap();
            let _portstr = portmatch.get(0).map_or("".to_string(), |m| m.as_str().to_string());
            let _port0: i32 = ftp::PORT_OCTI0.captures(&_portstr).unwrap().get(0).map_or(0, |m| m.as_str().parse().unwrap());
            let _port1: i32 = ftp::PORT_OCTI1.captures(&_portstr).unwrap().get(0).map_or(0, |m| m.as_str().parse().unwrap());
            let _port: i32 = (_port0*256)+_port1;

            // set IP and PORT.
            _user.data_ip = _ip;
            _user.data_port = _port;

            // reply
            ftp::send_reply(&mut _stream, &ftp::reply::COMMAND_OK.to_string(), &("PORT command successful."))?;
            return Ok(());
    }

    pub fn process_user_cmd(mut _stream: &mut TcpStream, mut _user: &mut ClientConnection, _cmd: &FtpCmd) ->
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

    pub fn process_pass_cmd(mut _stream: &mut TcpStream, mut _user: &mut ClientConnection, _cmd: &FtpCmd) ->
        Result<(), Box<dyn std::error::Error>> {
            // Do pre-checks.
            // Check if user is already logged in.
            if _user.is_user_logged {
                ftp::send_reply(&mut _stream, &ftp::reply::LOGGED_IN.to_string(), "Already Logged in.")?;
                return Ok(());
            }

            // Apply Command.
            _user.user.password = (&_cmd._args).to_string();
            _user.is_requesting_login = true;
            return Ok(());
        }

    pub fn process_syst_cmd(mut _stream: &mut TcpStream, mut _user: &mut ClientConnection, _cmd: &FtpCmd) ->
        Result<(), Box<dyn std::error::Error>> {
            ftp::send_reply(&mut _stream, &ftp::reply::NAME.to_string(), "UNIX Type: L8")?;
            return Ok(());
    }

    pub fn process_quit_cmd(mut _stream: &mut TcpStream, mut _user: &mut ClientConnection, _cmd: &FtpCmd) ->
        Result<(), Box<dyn std::error::Error>> {
            ftp::send_reply(&mut _stream, &ftp::reply::CLOSING.to_string(), "One off the racks.")?;
            return Ok(());
    }

}
