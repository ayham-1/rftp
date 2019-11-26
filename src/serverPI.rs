pub mod serverPI {
    use regex::Regex;
    use ftp::*;
    use defines::defines::{ServerInfo, ClientInfo, FTPModes, PortRange};
    use ftp_server::*;
    use auth::*;

    #[derive(Debug, Defualt)]
    pub struct ftpCmd {
        _cmd: String = "".to_string(),
        _args: String = "".to_string(),
    }

    pub fn parseftpCMD(_recieved: String) -> ftpCmd {
        let result: ftpCmd = ftpCmd::default();

        _recieved._cmd = getCommand(_recieved);
        _recieved._args = getArgs(_recieved);
        return _recieved;
    }
    
    pub fn applyCMD(_stream: &mut TcpStream, _user: &mut ftp_server::ClientConnection, _cmd: &ftpCmd) {
       match _cmd._cmd {
           "USER" => process_user_cmd(&_stream, &_user, &_cmd);
           _ => println!("Unkown Command"); // TODO: Report to client this being unkown cmd.
       }
    }
    
    pub fn process_user_cmd(_stream: &mut TcpStream, _user: &mut ftp_server::ClientConnection, _cmd: &ftpCmd) {
        // Do pre-checks.
        
        // Check if user is already logged in.
        if _user.is_user_logged == false {
            ftp::sendReply(&_stream, &ftp::reply::LOGGED_IN.to_string(), "Already Logged in.");
            return;
        }

        // Apply Comand.
    }
}
