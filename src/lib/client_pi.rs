pub mod client_pi {
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
            match cmd {
                "?" => help::cmd()?,
                "BYE" => quit::cmd(&mut _server_info)?,
                "QUIT" => quit::cmd(&mut _server_info)?,
                _ => {
                    return Err(ClientError::Regular(
                            ErrorKind::UnrecognizedCmd));
                }
            };
            return Ok(());
    }
}
