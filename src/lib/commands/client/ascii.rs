use crate::defines::defines::*;

pub fn cmd(_user: &mut ServerConnection) ->
Result<(), ClientError> {
    _user.data_type = FTPTypes::ASCII;
    return Err(ClientError::Regular(ErrorKind::NoWait));
}
