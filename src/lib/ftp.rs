pub mod ftp {
    use regex::Regex;
    use std::net::{TcpStream, TcpListener};
    use std::io::{Write};

    lazy_static! {
        pub static ref REPLY_CODE: Regex = Regex::new(r"\d\d\d\s").unwrap();
        pub static ref CMD_TYPE: Regex = Regex::new(r"\w{3,4}\s").unwrap();
        pub static ref CMD_ARGS: Regex = Regex::new(r"\s.+").unwrap();
        pub static ref REMOVE_SPACES: Regex = Regex::new(r"[^\s*].*[^\s*]").unwrap();
        pub static ref PORT_IP: Regex = Regex::new(r"(\d+),(\d+),(\d+),(\d+)").unwrap();
        pub static ref PORT_PRT: Regex = Regex::new(r"(\d+),(\d+)$").unwrap();
        pub static ref PORT_OCTI0: Regex = Regex::new(r"^(\d+)").unwrap();
        pub static ref PORT_OCTI1: Regex = Regex::new(r"(\d+)$").unwrap();
    }

    pub fn make_path_jailed(path: &str) -> String {
        if path.to_string().starts_with("/var/rftp/") {
            return path.to_string();
        }
        else {
            let mut result = String::new();
            result.push_str("/var/rftp/");
            result.push_str(path);
            return result;
        }
    }

    pub fn get_machine_ip() -> String {
        // get machine IP.
        let mut _address = String::new();
        for iface in 
            ifaces::Interface::get_all().unwrap()
                .into_iter() {
                    if !iface.name.contains("lo") {
                        if iface.kind == ifaces::Kind::Ipv4 {
                            _address = iface.addr.unwrap().to_string();
                        }
                    }
                }
        let _ip = str::replace(_address.as_str(), ".", ",");
        let mut result = String::new();
        // rmeove extra numbers from address.
        for c in _ip.chars() {
            if c != ':' {
                result.push_str(&c.to_string());
            }
            else {break;}
        }
        result.push_str(",");
        return result;
    }

    pub fn get_available_port() -> Option<u16> {
        (8000..9000)
            .find(|port| port_is_available(*port))
    }

    pub fn port_is_available(port: u16) -> bool {
        match TcpListener::bind(("0.0.0.0", port)) {
            Ok(_) => true,
            Err(_) => false,
        }
    }

    pub fn gen_reply(_code: &str, _info: &str) -> String {
        return String::from(_code.to_string() + " " + _info + "\r\n");
    }

    pub fn send_reply(_stream: &mut TcpStream, _code: &str, _info: &str) -> Result<(), std::io::Error> {
        _stream.write(gen_reply(_code, _info).as_bytes())?; 
        Ok(())
    }

    pub fn remove_whitespace(s: &mut String) {
        s.retain(|c| !c.is_whitespace());
    }

    pub fn get_command(_recieved: &String) -> String {
        let cmd = CMD_TYPE.captures(&_recieved).unwrap();
        let mut cmdstr = cmd.get(0).map_or("".to_string(), |m| m.as_str().to_string());
        remove_whitespace(&mut cmdstr);
        return cmdstr;
    }
    
    pub fn get_args(_recieved: &String) -> String {
        let args = CMD_ARGS.captures(&_recieved);
        if args.is_none() == true {
            return "".to_string();
        }
        let mut argsstr = args.unwrap().get(0).map_or("".to_string(), |m| m.as_str().to_string());
        remove_whitespace(&mut argsstr);
        return argsstr;
    }

    pub mod reply {
        // 1xx: Positive Preliminary Reply
        //pub const INITIATING: u32                  = 100;
        //pub const RESTART_MARKER: u32              = 110;
        //pub const READY_MINUTE: u32                = 120;
        //pub const ALREADY_OPEN: u32                = 125;
        pub const ABOUT_TO_SEND: u32               = 150;

        // 2xx: Positive Completion Reply
        pub const COMMAND_OK: u32                  = 200;
        pub const COMMAND_NOT_IMPLEMENTED: u32     = 202;
        //pub const SYSTEM: u32                      = 211;
        //pub const DIRECTORY: u32                   = 212;
        //pub const FILE: u32                        = 213;
        //pub const HELP: u32                        = 214;
        pub const NAME: u32                        = 215;
        pub const READY: u32                       = 220;
        pub const CLOSING: u32                     = 221;
        //pub const DATA_CONNECTION_OPEN: u32        = 225;
        pub const CLOSING_DATA_CONNECTION: u32     = 226;
        pub const PASSIVE_MODE: u32                = 227;
        //pub const LONG_PASSIVE_MODE: u32           = 228;
        //pub const EXTENDED_PASSIVE_MODE: u32       = 229;
        pub const LOGGED_IN: u32                   = 230;
        //pub const LOGGED_OUT: u32                  = 231;
        //pub const LOGOUT_ACK: u32                  = 232;
        //pub const AUTH_OK: u32                     = 234;
        pub const REQUESTED_FILE_ACTION_OK: u32    = 250;
        //pub const PATH_CREATED: u32                = 257;

        // 3xx: Positive intermediate Reply
        pub const NEED_PASSWORD: u32               = 331;
        //pub const LOGIN_NEED_ACCOUNT: u32          = 332;
        pub const REQUEST_FILE_PENDING: u32        = 350;

        // 4xx: Transient Negative Completion Reply
        //pub const NOT_AVAILABLE: u32               = 421;
        //pub const CANNOT_OPEN_DATA_CONNECTION: u32 = 425;
        //pub const TRANSER_ABORTED: u32             = 426;
        //pub const INVALID_CREDENTIALS: u32         = 430;
        //pub const HOST_UNAVAILABLE: u32            = 434;
        //pub const REQUEST_FILE_ACTION_IGNORED: u32 = 450;
        //pub const ACTION_ABORTED: u32              = 451;
        pub const REQUESTED_ACTION_NOT_TAKEN: u32  = 452;

        // 5xx: Permanent Negative Completion Reply
        //pub const BAD_COMMAND: u32                 = 500;
        pub const BAD_ARGUMENTS: u32               = 501;
        //pub const NOT_IMPLEMENTED: u32             = 502;
        //pub const BAD_SEQUENCE: u32                = 503;
        //pub const NOT_IMPLEMENTED_PARAMETER: u32   = 504;
        pub const NOT_LOGGED_IN: u32               = 530;
        //pub const STORING_NEED_ACCOUNT: u32        = 532;
        pub const FILE_UNAVAILABLE: u32            = 550;
        //pub const PAGE_TYPE_UNKNOWN: u32           = 551;
        //pub const EXCEEDED_STORAGE: u32            = 552;
        //pub const BAD_FILENAME: u32                = 553;
    }
}
