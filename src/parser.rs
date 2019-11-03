include! { "defines.rs" }

mod parser {
    extern crate clap;
    use defines::{ServerInfo, ClientInfo, FTPModes, PortRange};

    pub fn parse_server_info(_args: &clap::ArgMatches) -> ServerInfo {
        let result = ServerInfo {
            mode: FTPModes::Active,
            max_connections: 10,
            port_range: PortRange {x:2048,y:10240},
            allow_anonymous: false,
            log_file: "stdout".to_string()
        };
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
