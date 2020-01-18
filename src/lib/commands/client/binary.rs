use crate::defines::defines::*;

pub fn cmd(_user: &mut ServerConnection) ->
Result<(), ClientError> {
    _user.data_type = FTPTypes::BINARY;
    return Err(ClientError::Regular(ErrorKind::NoWait));
}
