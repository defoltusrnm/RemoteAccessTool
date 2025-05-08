use crate::error_handling::server_errors::ServerError;

pub struct NetMessage {
    bytes: Vec<u8>,
}

impl NetMessage {
    pub fn new(bytes: Vec<u8>) -> NetMessage {
        NetMessage { bytes }
    }

    pub fn to_string(&self) -> Result<String, ServerError> {
        String::from_utf8(self.bytes.to_owned()).map_err(|err| ServerError::new(err.to_string()))
    }

    pub const fn bytes(&self) -> &Vec<u8> {
        &self.bytes
    }
}
