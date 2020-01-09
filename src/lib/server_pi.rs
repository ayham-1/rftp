pub mod server_pi {
    use std::net::{Shutdown, TcpStream};
    use crate::ftp::*;
    use crate::defines::defines::*;
    use std::process::Command;
    use std::io::Write;
    use net2::TcpBuilder;

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
            "TYPE" => process_type_cmd(&mut _stream, &mut _user, &_cmd)?,
            "LIST" => process_list_cmd(&mut _stream, &mut _user, &_cmd)?,
            "HELP" => process_help_cmd(&mut _stream, &mut _user, &_cmd)?,
            _ => { 
                ftp::send_reply(&mut _stream, &ftp::reply::COMMAND_NOT_IMPLEMENTED.to_string(), "Command Not Implemented.")?;
            }
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
        // Open data connection.
        ftp::send_reply(&mut _stream, &ftp::reply::ABOUT_TO_SEND.to_string(), "Opening ASCII Data connection.")?;
        let address = &mut _user.data_ip;
        address.push_str(":");
        address.push_str(_user.data_port.to_string().as_str());
        _user.data_conc = TcpBuilder::new_v4().unwrap().reuse_address(true).unwrap().bind("0.0.0.0:20").unwrap().connect(address.as_str()).unwrap();

        if _cmd._args == "" {
            let result = Command::new("ls")
                .arg("-l")
                .output().expect("ls command not found.");
            _user.data_conc.write(&result.stdout)?;
        } else {
            let result = Command::new("ls")
                .arg("-l")
                .arg(&_cmd._args)
                .output().expect("ls command not found.");
            _user.data_conc.write(&result.stdout)?;
        }
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
        ftp::send_reply(&mut _stream, &ftp::reply::CLOSING.to_string(), "Few, one off the racks.")?;
        return Ok(());
    }

}
