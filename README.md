# rftp
A simple CLI ftp client/server for a school project written in rust.

## Features

### Server:
* Users can have seperate permissions, (List, Read, All, Nothing)
* Server can run in passive, active or both modes of connection.
* Server can handle multiple users.

## Getting Started

### Consumers:
Consumers should look at the releases page for binaries.

### Developers:

#### Prerequisites:
Rust can be installed from [here](https://www.rust-lang.org/tools/install).

#### Building:
```sh
cargo build
```

##### Running:

For commandline help:
```sh
sudo ./target/debug/rftp -h
```
All subcommands have ```-h``` command.

To run the server:
```sh
sudo ./target/debug/rftp server
```

To run the database manager
```sh
sudo ./target/debug/rftp db
```

## Built With:
* Rust: [Language](https://www.rust-lang.org/tools/install)

## Authors:
* **altffour** - *Initial work* - [realaltffour](https://github.com/realaltffour)
