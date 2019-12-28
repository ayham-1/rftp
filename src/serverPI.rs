pub mod serverPI {
    use std::net::{TcpStream};
    use ftp::*;
    use ftp_server::*;

    #[derive(Debug, Default)]
    pub struct FtpCmd {
        pub _cmd: String,
        pub _args: String,
    }

    pub fn parseftpCMD(_recieved: String) -> FtpCmd {
        let mut result: FtpCmd = FtpCmd::default();

        result._cmd = ftp::getCommand(&_recieved);
        result._args = ftp::getArgs(&_recieved);
        return result;
    }
    
    pub fn applyCMD(mut _stream: &mut TcpStream, mut _user: &mut ftp_server::ClientConnection, _cmd: &FtpCmd) {
        let cmd = _cmd._cmd.as_str();
        match cmd {
            "USER" => process_USER_cmd(&mut _stream, &mut _user, &_cmd),
            "PASS" => process_PASS_cmd(&mut _stream, &mut _user, &_cmd),
            "SYST" => process_SYST_cmd(&mut _stream, &mut _user, &_cmd),
            _ => { 
                ftp::sendReply(&mut _stream, &ftp::reply::COMMAND_NOT_IMPLEMENTED.to_string(), "Command Not Implemented.");
            }
        }
    }
    
    pub fn process_USER_cmd(mut _stream: &mut TcpStream, mut _user: &mut ftp_server::ClientConnection, _cmd: &FtpCmd) {
        // Do pre-checks.
        
        // Check if user is already logged in.
        if _user.is_user_logged {
            ftp::sendReply(&mut _stream, &ftp::reply::LOGGED_IN.to_string(), "Already Logged in.");
            return;
        }

        // Apply Command.
        _user.user.username = (&_cmd._args).to_string();
        
        // Check if anonymous.
        if _user.user.username == "anonymous" {
            _user.is_anon = true;
            _user.is_requesting_login = true;
            ftp::sendReply(&mut _stream, &ftp::reply::LOGGED_IN.to_string(), &("User logged in as anonymous."));
            return;
        }
        else {
            _user.is_anon = false;
        }

        ftp::sendReply(&mut _stream, &ftp::reply::NEED_PASSWORD.to_string(), &("User ".to_owned() + _user.user.username.as_str() + " needs password."));
    }

    pub fn process_PASS_cmd(mut _stream: &mut TcpStream, mut _user: &mut ftp_server::ClientConnection, _cmd: &FtpCmd) {
        // Do pre-checks.
        // Check if user is already logged in.
        if _user.is_user_logged {
            ftp::sendReply(&mut _stream, &ftp::reply::LOGGED_IN.to_string(), "Already Logged in.");
            return;
        }

        // Apply Command.
        _user.user.password = (&_cmd._args).to_string();
        _user.is_requesting_login = true;
    }

    pub fn process_SYST_cmd(mut _stream: &mut TcpStream, mut _user: &mut ftp_server::ClientConnection, _cmd: &FtpCmd) {
        ftp::sendReply(&mut _stream, &ftp::reply::NAME.to_string(), "UNIX Type: L8");
    }


}
