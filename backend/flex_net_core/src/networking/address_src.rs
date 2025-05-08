use crate::error_handling::server_errors::ServerError;

pub struct EndpointAddress {
    pub host: String,
    pub port: i32,
}

impl EndpointAddress {
    pub fn from_ip_and_port(host: String, port: i32) -> EndpointAddress {
        EndpointAddress { host, port }
    }
}

pub trait EndpointAddressSrc {
    fn get(self) -> Result<EndpointAddress, ServerError>;
}
