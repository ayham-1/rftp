use crate::defines::defines::*;

pub fn cmd(mut _server_info: &mut ServerConnection) ->
Result<(), ClientError> {
    _server_info.is_closing = true;
    return Err(ClientError::Regular(ErrorKind::NoWait));
}
