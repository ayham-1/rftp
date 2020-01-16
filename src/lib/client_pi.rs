pub mod client_pi {
    use crate::defines::defines::*;
    use std::net::TcpStream;

    type Result<T> = std::result::Result<T, ClientError>;

    pub fn send_cmd(mut _stream: &mut TcpStream, mut _cmd: &str) -> 
        Result<()> {
            // Pre-checks.

            // strip new line stuff.
            let _stripped = _cmd.replace('\n', "");
            
            // uppercase all.
            let uppercmd = _stripped.to_uppercase().to_owned()
                .to_string();
            let cmd = uppercmd.as_str();

            // Despatch commands.
            match cmd {
                "QUIT" => println!("Hello"),
                _ => {
                    return Err(ClientError::Regular(
                            ErrorKind::UnrecognizedCmd));
                }
            }
            return Ok(());
    }
}
