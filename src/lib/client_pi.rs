pub mod client_pi {
    use std::net::TcpStream;
    use std::error::Error;
    use std::fmt;

    type Result<T> = std::result::Result<T, ClientError>;

    #[derive(Debug, PartialEq)]
    pub enum ClientError {
        // External libraires errors.

        // client_pi errors.
        Regular(ErrorKind)
    }
    impl Error for ClientError {
        fn description(&self) -> &str {
            match *self {
                ClientError::Regular(ref err) => err.as_str()
            }
        }
    }
    impl fmt::Display for ClientError {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            match *self {
                ClientError::Regular(ref err) => 
                    write!(f, "A client error occured: {:?}", err),
            }
        }
    }

    #[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    pub enum ErrorKind {
        UnrecognizedCmd,
    }
    impl ErrorKind {
        fn as_str(&self) -> &str {
            match *self {
                ErrorKind::UnrecognizedCmd => "unrecognized command.",
            }
        }
    }

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
