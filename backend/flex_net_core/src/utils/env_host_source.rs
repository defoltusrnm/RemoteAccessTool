use std::env;

use crate::{error_handling::server_errors::ServerError, networking::address_src::{EndpointAddress, EndpointAddressSrc}};


pub struct EnvEndpointAddressSrc {
    default_port: i32
}

impl EnvEndpointAddressSrc {
    pub fn new_with_port_fallback(port: i32) -> Self {
        EnvEndpointAddressSrc {
            default_port: port
        }
    }
}

impl EndpointAddressSrc for EnvEndpointAddressSrc {

    fn get(self) -> Result<EndpointAddress, ServerError> {
        env::var("HOST")
            .map_err(|err| ServerErrors::host_is_not_provided(err))
            .map(|host| {
                let port = env::var("PORT")
                    .map_err(|x| ServerErrors::port_env_read_error(x))
                    .and_then(|x| {
                        x.parse::<i32>()
                            .map_err(|err| ServerErrors::port_parse_error(err))
                    })
                    .unwrap_or_else(|err| {
                        log::error!(
                            "error occurred when reading port \"{err}\", will take default"
                        );
                        self.default_port
                    });

                EndpointAddress::from_ip_and_port(host, port)
            })
    }
}

struct ServerErrors;

impl ServerErrors {
    pub fn host_is_not_provided(err: std::env::VarError) -> ServerError {
        ServerError::new(format!("host addr could not be provided: {err}"))
    }

    pub fn port_env_read_error(err: std::env::VarError) -> ServerError {
        ServerError::new(format!("could not read port from env: {err}"))
    }

    pub fn port_parse_error(err: std::num::ParseIntError) -> ServerError {
        ServerError::new(format!("could not parse port from env: {err}"))
    }
}
