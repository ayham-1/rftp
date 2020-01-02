pub mod server_pi {
    use std::net::{TcpStream};
    use crate::ftp::*;
    use crate::ftp_server::*;

    #[derive(Debug, Default)]
    pub struct FtpCmd {
        pub _cmd: String,
        pub _args: String,
    }

    pub fn parseftp_cmd(_recieved: String) -> FtpCmd {
        let mut result: FtpCmd = FtpCmd::default();

        result._cmd = ftp::get_command(&_recieved);
        result._args = ftp::get_args(&_recieved);
        return result;
    }
    
    pub fn apply_cmd(mut _stream: &mut TcpStream, mut _user: &mut ftp_server::ClientConnection, _cmd: &FtpCmd) ->
        Result<(), Box<dyn std::error::Error>> {
        let cmd = _cmd._cmd.as_str();
        match cmd {
            "USER" => process_user_cmd(&mut _stream, &mut _user, &_cmd)?,
            "PASS" => process_pass_cmd(&mut _stream, &mut _user, &_cmd)?,
            "SYST" => process_syst_cmd(&mut _stream, &mut _user, &_cmd)?,
            "QUIT" => process_quit_cmd(&mut _stream, &mut _user, &_cmd)?,
            "PORT" => process_port_cmd(&mut _stream, &mut _user, &_cmd)?,
            _ => { 
                ftp::send_reply(&mut _stream, &ftp::reply::COMMAND_NOT_IMPLEMENTED.to_string(), "Command Not Implemented.")?;
            }
        }
        return Ok(());
    }

    pub fn process_port_cmd(mut _stream: &mut TcpStream, mut _user: &mut ftp_server::ClientConnection, _cmd: &FtpCmd) ->
        Result<(), Box<dyn std::error::Error>> {
        // get IP.
        let ipmatch = ftp::PORT_IP.captures(&_cmd._args).unwrap();
        let _ip = ipmatch.get(0).map_or("".to_string(), |m| m.as_str().to_string());
        // get PORT.
        let portmatch = ftp::PORT_PRT.captures(&_cmd._args).unwrap();
        let _portstr = portmatch.get(0).map_or("".to_string(), |m| m.as_str().to_string());
        let _portoctis = ftp::PORT_OCTI.captures(&_portstr).unwrap();
        let _port0: i32 = _portoctis.get(0).map_or(0, |m| m.as_str().parse().unwrap());
        let _port1: i32 = _portoctis.get(1).map_or(0, |m| m.as_str().parse().unwrap());
        let _port: i32 = (_port0*256)+_port1;

        // set IP and PORT.
        _user.data_ip = _ip;
        _user.data_port = _port;

        // reply
        ftp::send_reply(&mut _stream, &ftp::reply::COMMAND_OK.to_string(), &("PORT command successful."))?;
        return Ok(());
    }

    pub fn process_user_cmd(mut _stream: &mut TcpStream, mut _user: &mut ftp_server::ClientConnection, _cmd: &FtpCmd) ->
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

    pub fn process_pass_cmd(mut _stream: &mut TcpStream, mut _user: &mut ftp_server::ClientConnection, _cmd: &FtpCmd) ->
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

    pub fn process_syst_cmd(mut _stream: &mut TcpStream, mut _user: &mut ftp_server::ClientConnection, _cmd: &FtpCmd) ->
        Result<(), Box<dyn std::error::Error>> {
        ftp::send_reply(&mut _stream, &ftp::reply::NAME.to_string(), "UNIX Type: L8")?;
        return Ok(());
    }

    pub fn process_quit_cmd(mut _stream: &mut TcpStream, mut _user: &mut ftp_server::ClientConnection, _cmd: &FtpCmd) ->
        Result<(), Box<dyn std::error::Error>> {
        ftp::send_reply(&mut _stream, &ftp::reply::CLOSING.to_string(), "Few, one off the racks.")?;
        return Ok(());
    }

}
