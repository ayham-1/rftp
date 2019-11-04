include! { "defines.rs" }

mod parser {
    extern crate clap;
    use defines::{ServerInfo, ClientInfo, FTPModes, PortRange};

    pub fn parse_server_info(_args: &clap::ArgMatches) -> ServerInfo {
        let mut result = ServerInfo {
            mode: FTPModes::Active,
            max_connections: 10,
            port_range: PortRange {x:2048,y:10240},
            allow_anonymous: false,
            log_file: "stdout".to_string()
        };

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

        result
    }

    pub fn parse_client_info(_args: &clap::ArgMatches) -> ClientInfo {
        let result = ClientInfo {
            server_name: "localhost".to_string(),
            connect_mode: FTPModes::Active,
            username: "root".to_string(),
            password: "toor".to_string()
        };
        result
    }
}
