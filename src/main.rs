extern crate clap;
extern crate regex;
#[macro_use]
extern crate lazy_static;
extern crate serde_json;
extern crate serde;

mod lib;
use crate::lib::*;

use crate::defines::defines::{ClientInfo};
use clap::{Arg, App, SubCommand};
use crate::parser::parser::{parse_server_info, parse_client_info, parse_dbcmd_info};
use crate::ftp_server::ftp_server::{start_server};
use crate::db::db::apply_dbcmd;

fn run(_args: clap::ArgMatches) -> Result<(), Box<dyn std::error::Error>> {
    match _args.subcommand() {
        ("server", Some(m)) => {
            let _info = parse_server_info(m); 
            start_server(_info)?;
        },
        ("client", Some(m)) => {
            let _info: ClientInfo = parse_client_info(m); 
        },
        ("db", Some(m)) => {
            let cmd = parse_dbcmd_info(m);
            apply_dbcmd(&cmd)?;
            println!("Command Processed.");
        },
        _ => println!("Specify running mode."),
    }
    Ok(())
}

fn main() {
    let _args = App::new("ftp")
        .version("0.1.4")
        .author("realaltffour <ayhamaboualfadl@gmail.com>")
        .about("A ftp client/server that makes file transfers easy.")
        .subcommand(SubCommand::with_name("server")
            .about("Starts app in server mode.")
        .arg(Arg::with_name("mode")
            .short("m")
            .long("mode")
            .takes_value(true)
            .value_name("active/passive/both/1/2/3")
            .default_value("both")
            .help("Whether to accept passive, active or both connections.")
            .required(true))
        .arg(Arg::with_name("max-connections")
            .short("c")
            .long("max-connections")
            .takes_value(true)
            .default_value("10")
            .value_name("num")
            .help("Maximum number of client connections to accept.")
            .required(false))
        .arg(Arg::with_name("port-range")
            .short("p")
            .long("ports")
            .takes_value(true)
            .value_name("x-y")
            .default_value("2048-10240")
            .help("Range of unprivilged ports to allow clients to connect to.")
            .required(false))
        .arg(Arg::with_name("anonymous-access")
            .short("a")
            .long("anonymous")
            .takes_value(false)
            .help("Whether to allow anonymous access or not.")
            .required(false))
        .arg(Arg::with_name("log")
            .short("l")
            .long("log")
            .value_name("FILE")
            .takes_value(true)
            .help("Location to output log to.")
            .required(false)))
        .subcommand(SubCommand::with_name("client")
            .about("Starts app in client mode.")
            .arg(Arg::with_name("server-name")
                .short("n")
                .long("server-name")
                .takes_value(true)
                .value_name("NAME/IP")
                .default_value("localhost")
                .help("Server name/IP to connect to.")
                .required(true))
            .arg(Arg::with_name("connect-mode")
                .short("e")
                .long("connect-mode")
                .takes_value(true)
                .value_name("active/passive")
                .default_value("passive")
                .help("Connection mode, default tries both.")
                .required(true))
            .arg(Arg::with_name("username")
                .short("u")
                .long("user")
                .takes_value(true)
                .value_name("userstr")
                .help("User to login with.")
                .required(false))
            .arg(Arg::with_name("password")
                .short("w")
                .long("pass")
                .takes_value(true)
                .value_name("pass")
                .help("Password to login with.")
                .required(false)))
        .subcommand(SubCommand::with_name("db")
            .about("Controls local user database")
            .subcommand(SubCommand::with_name("add")
                .about("Adds a user to the database")
                .arg(Arg::with_name("name")
                    .long("name")
                    .short("l")
                    .takes_value(true)
                    .help("Name of the user.")
                    .required(true))
                .arg(Arg::with_name("pass")
                    .long("pass")
                    .short("q")
                    .takes_value(true)
                    .help("Password of the user.")
                    .required(true))
                .arg(Arg::with_name("access-rights")
                    .short("r")
                    .long("rights")
                    .takes_value(true)
                    .help("Level of access the user can have.")
                    .value_name("list/read/all/none/0/1/2/3/4")
                    .default_value("None")
                    .required(true)))
            .subcommand(SubCommand::with_name("rm")
                    .about("Removes user from the database.")
                    .arg(Arg::with_name("name")
                        .long("name")
                        .takes_value(true)
                        .help("Name of the user.")
                        .required(true)))
            .arg(Arg::with_name("list")
                .short("ls")
                .long("list")
                .takes_value(false)
                .help("List all users."))
            .subcommand(SubCommand::with_name("clean")
                .help("Cleans user's database.")))
        .get_matches();
    match run(_args) {
        Ok(_v) => {
            println!("Exiting with no errors...");
        }
        Err(_e) => {
            println!("Exiting with errors... \n Error: {}", _e);
        }
    }
}
