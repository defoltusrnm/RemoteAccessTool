use std::env;

use anyhow::Context;

use crate::networking::address_src::{EndpointAddress, EndpointAddressSrc};

pub struct EnvEndpointAddressSrc {
    default_port: i32,
}

impl EnvEndpointAddressSrc {
    pub fn new_with_port_fallback(port: i32) -> Self {
        EnvEndpointAddressSrc { default_port: port }
    }
}

impl EndpointAddressSrc for EnvEndpointAddressSrc {
    fn get(&self) -> Result<EndpointAddress, anyhow::Error> {
        let host = env::var("HOST").with_context(|| "Failed to get host")?;

        let port = env::var("PORT")
            .with_context(|| "Failed to get port")
            .and_then(|x| {
                x.parse::<i32>()
                    .with_context(|| format!("Failed to parse {x}"))
            })
            .unwrap_or_else(|err| {
                log::warn!("cannot parse port: {err}");
                self.default_port
            });

        Ok(EndpointAddress::from_ip_and_port(&host, port))
    }
}
