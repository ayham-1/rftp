pub mod client_pi {
    use std::net::TcpStream;
    use std::error;
    use std::fmt;

    type Result<T> = std::result::Result<T, Box<dyn error::Error>>;

    #[derive(Debug, Clone)]
    struct UnrecognizedCmd;
    impl fmt::Display for UnrecognizedCmd {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "command not recognized.")
        }
    }

    impl error::Error for UnrecognizedCmd {
        fn source(&self) -> Option<&(dyn error::Error + 'static)> {
            None
        }
    }

    pub fn send_cmd(mut _stream: &mut TcpStream, mut _cmd: &str) -> 
        Result<()> {
            // Pre-checks.
            
            // uppercase all.
            let uppercmd = _cmd.to_string().to_uppercase().to_owned();
            let cmd = uppercmd.as_str();

            // Despatch commands.
            match cmd {
                _ => {
                    return Err(Box::new(UnrecognizedCmd));
                }
            };
            return Ok(());
    }
}
