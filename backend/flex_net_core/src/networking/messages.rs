use anyhow::Context;

pub struct NetMessage {
    bytes: Vec<u8>,
}

impl NetMessage {
    pub fn new(bytes: Vec<u8>) -> NetMessage {
        NetMessage { bytes }
    }

    pub fn to_string(&self) -> Result<String, anyhow::Error> {
        String::from_utf8(self.bytes.to_owned()).with_context(|| "Failed to parse u8 bytes")
    }

    pub const fn bytes(&self) -> &Vec<u8> {
        &self.bytes
    }
}
