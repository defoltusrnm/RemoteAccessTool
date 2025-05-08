use std::fmt::Display;

pub struct EndpointAddress {
    pub host: String,
    pub port: i32,
}

impl EndpointAddress {
    pub fn from_ip_and_port(host: &impl Display, port: i32) -> EndpointAddress {
        EndpointAddress {
            host: host.to_string(),
            port,
        }
    }
}

pub trait EndpointAddressSrc {
    fn get(&self) -> Result<EndpointAddress, anyhow::Error>;
}
