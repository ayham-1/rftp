pub mod client_pi {
    use crate::ftp::*;
    use crate::defines::defines::*;
    use crate::lib::commands::client::*;
    use std::net::TcpStream;

    type Result<T> = std::result::Result<T, ClientError>;

    pub fn send_cmd(mut _stream: &mut TcpStream, mut _cmd: &str,
        mut _server_info: &mut ServerConnection) -> 
        Result<()> {
            // Pre-checks.

            // strip new line stuff.
            let _stripped = _cmd.replace('\n', "");
            
            // uppercase all.
            let uppercmd = _stripped.to_uppercase().to_owned()
                .to_string();

            let cmd = uppercmd.as_str();

            // Dispatch commands.
            match ftp::get_command(&cmd.to_string()).as_str() {
                "?" => help::cmd()?,
                "BYE" => quit::cmd(&mut _server_info)?,
                "QUIT" => quit::cmd(&mut _server_info)?,
                "CD" => cd::cmd(&mut _stream, _cmd)?,
                "CDUP" => cdup::cmd(&mut _stream, _cmd)?,
                "DELETE" => delete::cmd(&mut _stream, _cmd)?,
                "RENAME" => rename::cmd(&mut _stream, _cmd)?,
                "MKD" => mkd::cmd(&mut _stream, _cmd)?,
                "RMD" => rmd::cmd(&mut _stream, _cmd)?,
                "APPEND" => append::cmd(&mut _stream, _cmd, 
                    &mut _server_info)?,
                "PUT" => put::cmd(&mut _stream, _cmd, 
                    &mut _server_info)?,
                "GET" => get::cmd(&mut _stream, _cmd, 
                    &mut _server_info)?,
                "SYSTEM" => system::cmd(&mut _stream)?,
                "PASSIVE" => passive::cmd(&mut _server_info)?,
                "ACTIVE" => active::cmd(&mut _server_info)?,
                "ASCII" => ascii::cmd(&mut _server_info)?,
                "BINARY" => binary::cmd(&mut _server_info)?,
                _ => {
                    return Err(ClientError::Regular(
                            ErrorKind::UnrecognizedCmd));
                }
            };
            return Ok(());
    }
}
