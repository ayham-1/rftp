pub mod serverPI {
    use regex::Regex;
    use std::net::{TcpListener, TcpStream};
    use ftp::*;
    use defines::defines::{ServerInfo, ClientInfo, FTPModes, PortRange};
    use ftp_server::*;
    use db::*;

    #[derive(Debug, Default)]
    pub struct ftpCmd {
        pub _cmd: String,
        pub _args: String,
    }

    pub fn parseftpCMD(_recieved: String) -> ftpCmd {
        let mut result: ftpCmd = ftpCmd::default();

        result._cmd = ftp::getCommand(&_recieved);
        result._args = ftp::getArgs(&_recieved);
        return result;
    }
    
    pub fn applyCMD(mut _stream: &mut TcpStream, mut _user: &mut ftp_server::ClientConnection, _cmd: &ftpCmd) {
        let cmd = _cmd._cmd.as_str();
        match cmd {
            "USER" => process_USER_cmd(&mut _stream, &mut _user, &_cmd),
            "PASS" => process_PASS_cmd(&mut _stream, &mut _user, &_cmd),
            "SYST" => ftp::sendReply(&mut _stream, &ftp::reply::COMMAND_NOT_IMPLEMENTED.to_string(), "Command Not Implemented."),
            _ => { 
                ftp::sendReply(&mut _stream, &ftp::reply::COMMAND_NOT_IMPLEMENTED.to_string(), "Command Not Implemented.");
            }
        }
    }
    
    pub fn process_USER_cmd(mut _stream: &mut TcpStream, mut _user: &mut ftp_server::ClientConnection, _cmd: &ftpCmd) {
        // Do pre-checks.
        
        // Check if user is already logged in.
        if _user.is_user_logged {
            ftp::sendReply(&mut _stream, &ftp::reply::LOGGED_IN.to_string(), "Already Logged in.");
            return;
        }

        // Apply Command.
        _user.user.username = (&_cmd._args).to_string();

        // TODO: Check if anonymous access is allowed.
        ftp::sendReply(&mut _stream, &ftp::reply::NEED_PASSWORD.to_string(), &("User ".to_owned() + _user.user.username.as_str() + " needs password."));
    }

    pub fn process_PASS_cmd(mut _stream: &mut TcpStream, mut _user: &mut ftp_server::ClientConnection, _cmd: &ftpCmd) {
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
}
