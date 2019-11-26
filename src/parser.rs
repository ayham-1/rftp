pub mod parser {
    extern crate clap;
    use defines::defines::{ServerInfo, ClientInfo, FTPModes, PortRange};
    use db::*;
    use auth::*;

    pub fn parse_server_info(_args: &clap::ArgMatches) -> ServerInfo {
        let mut result = ServerInfo {
            mode: FTPModes::Active,
            max_connections: 10,
            port_range: PortRange {x:2048,y:10240},
            allow_anonymous: false,
            log_file: "stdout".to_string(),
            pwd: "".to_string()
        };
        let _pwd = std::env::current_dir().unwrap();
        result.pwd = _pwd.into_os_string().into_string().unwrap();

        let _in_mode = _args.value_of("mode").unwrap_or("both");
        let _in_max_connections = _args.value_of("max-connections").unwrap_or("10");
        let _in_port_range = _args.value_of("port-range").unwrap_or("2048-10240");
        let mut _in_anonymous_access_support: bool = false;
        if _args.is_present("anonymous-access") {
            _in_anonymous_access_support = true;
        }
        let _in_log_file = _args.value_of("log").unwrap_or("stdout");

        if _in_mode == "active" || _in_mode == "1" {
            result.mode = FTPModes::Active;
        } else if _in_mode == "passive" || _in_mode == "2" {
            result.mode = FTPModes::Passive;
        } else if _in_mode == "both" || _in_mode == "3" {
            result.mode = FTPModes::Both;
        } else {
            panic!("Unrecognized argument given to --mode");
        }

        result.max_connections = _in_max_connections.parse::<i32>().unwrap();

        let mut _in_port_range_x: String = "".to_string();
        let mut _in_port_range_y: String = "".to_string();
        let mut _xturn: bool = true;

        for c in _in_port_range.chars() {
            if c == '-' {
                _xturn = !_xturn;
                continue;
            }
            if _xturn {
                _in_port_range_x.push(c);
            } else {
                _in_port_range_y.push(c);
            }
        }

        result.port_range.x = _in_port_range_x.parse::<i32>().unwrap();
        result.port_range.y = _in_port_range_y.parse::<i32>().unwrap();
        result.allow_anonymous = _in_anonymous_access_support;
        result.log_file = _in_log_file.to_string();

        return result;
    }

    pub fn parse_client_info(_args: &clap::ArgMatches) -> ClientInfo {
        let mut result = ClientInfo {
            server_name: "localhost".to_string(),
            connect_mode: FTPModes::Passive,
            username: "root".to_string(),
            password: "toor".to_string()
        };

        let _in_mode = _args.value_of("connect-mode").unwrap_or("both");

        if _in_mode == "active" || _in_mode == "1" {
            result.connect_mode = FTPModes::Active;
        } else if _in_mode == "passive" || _in_mode == "2" {
            result.connect_mode = FTPModes::Passive;
        } else if _in_mode == "both" || _in_mode == "3" {
            result.connect_mode = FTPModes::Both;
        } else {
            panic!("Unrecognized argument given to --mode");
        }

        result.server_name = _args.value_of("server-name").unwrap_or("localhost").to_string();
        result.username = _args.value_of("username").unwrap_or("root").to_string();
        result.password = _args.value_of("password").unwrap_or("toor").to_string();

        return result;
    }

    pub fn parse_dbcmd_info(_args: &clap::ArgMatches) -> db::DBCmd {
        let mut result: db::DBCmd = db::DBCmd::default();
        
        if _args.is_present("add") {
            result.job = db::CmdJob::Add;
            result.user = _args.subcommand_matches("add").unwrap().value_of("name").unwrap().to_string();
            result.pass = _args.subcommand_matches("add").unwrap().value_of("pass").unwrap().to_string();
            let _in_rights = _args.subcommand_matches("add").unwrap().value_of("access-rights").unwrap().to_string();
            if _in_rights == String::from("list") || _in_rights == String::from("0"){
                result.rights = auth::Rights::List;
            }
            if _in_rights == String::from("read") || _in_rights == String::from("1"){
                result.rights = auth::Rights::Read;
            }
            if _in_rights == String::from("all") || _in_rights == String::from("2"){
                result.rights = auth::Rights::All;
            }
            if _in_rights == String::from("none") || _in_rights == String::from("3"){
                result.rights = auth::Rights::Nothing;
            }
            return result;
        } else if _args.is_present("rm") {
            result.job = db::CmdJob::Remove;
            result.user = _args.subcommand_matches("add").unwrap().value_of("name").unwrap().to_string();
            return result;
        } else if _args.is_present("list") {
            result.job = db::CmdJob::List;
            return result;
        } else if _args.is_present("clean") {
            result.job = db::CmdJob::Clean;
            return result;
        }
        return result;
    }
}
