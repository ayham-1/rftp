pub mod server_pi {
    use std::net::TcpStream;
    use crate::ftp::*;
    use crate::defines::defines::*;
    use crate::lib::commands::*;

    pub fn parseftp_cmd(_recieved: String) -> FtpCmd {
        let mut result: FtpCmd = FtpCmd::default();
        if _recieved == "" { return result; }

        result._cmd = ftp::get_command(&_recieved);
        result._args = ftp::get_args(&_recieved);
        return result;
    }

    pub fn apply_cmd(mut _stream: &mut TcpStream, 
        mut _user: &mut ClientConnection, _cmd: &FtpCmd) ->
        Result<(), Box<dyn std::error::Error>> {
            let cmd = _cmd._cmd.as_str();
            match cmd {
                "USER" => user::cmd(&mut _stream, &mut _user, &_cmd)?,
                "PASS" => pass::cmd(&mut _stream, &mut _user, &_cmd)?,
                "SYST" => syst::cmd(&mut _stream, &mut _user, &_cmd)?,
                "QUIT" => quit::cmd(&mut _stream, &mut _user, &_cmd)?,
                "PORT" => port::cmd(&mut _stream, &mut _user, &_cmd)?,
                "PASV" => pasv::cmd(&mut _stream, &mut _user, &_cmd)?,
                "TYPE" => r#type::cmd(&mut _stream, &mut _user, &_cmd)?,
                "LIST" => list::cmd(&mut _stream, &mut _user, &_cmd)?,
                "HELP" => help::cmd(&mut _stream, &mut _user, &_cmd)?,
                "NOOP" => noop::cmd(&mut _stream, &mut _user, &_cmd)?,
                "CWD" =>  cwd::cmd(&mut _stream, &mut _user, &_cmd)?,
                "MODE" => mode::cmd(&mut _stream, &mut _user, &_cmd)?,
                "ACCT" => acct::cmd(&mut _stream, &mut _user, &_cmd)?,
                "STRU" => stru::cmd(&mut _stream, &mut _user, &_cmd)?,
                "STOR" => stor::cmd(&mut _stream, &mut _user, &_cmd)?,
                "ABOR" => abor::cmd(&mut _stream, &mut _user, &_cmd)?,
                "ALLO" => allo::cmd(&mut _stream, &mut _user, &_cmd)?,
                "APPE" => appe::cmd(&mut _stream, &mut _user, &_cmd)?,
                "DELE" => dele::cmd(&mut _stream, &mut _user, &_cmd)?,
                "NLST" => nlst::cmd(&mut _stream, &mut _user, &_cmd)?,
                "REIN" => rein::cmd(&mut _stream, &mut _user, &_cmd)?,
                "RETR" => retr::cmd(&mut _stream, &mut _user, &_cmd)?,
                "RNFR" => rnfr::cmd(&mut _stream, &mut _user, &_cmd)?,
                "RNTO" => rnto::cmd(&mut _stream, &mut _user, &_cmd)?,
                "STAT" => stat::cmd(&mut _stream, &mut _user, &_cmd)?,
                "SITE" => site::cmd(&mut _stream, &mut _user, &_cmd)?,
                _ => { 
                    ftp::send_reply(&mut _stream,
                        &ftp::reply::COMMAND_NOT_IMPLEMENTED.to_string(),
                        "Command Not Implemented.")?;
                }
            }
            return Ok(());
    }
}
