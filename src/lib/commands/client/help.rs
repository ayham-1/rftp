use crate::defines::defines::*;

pub fn cmd() -> Result<(), ClientError> {
    println!("quit\tbye\tcd");
    println!("cdup\tdelete\trename");
    println!("mkd\trmd\tappend");
    println!("put\tget\tsystem");
    println!("ascii\tbinary\tquit");
    return Err(ClientError::Regular(ErrorKind::NoWait));
}
