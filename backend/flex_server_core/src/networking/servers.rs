use flex_net_core::networking::{address_src::EndpointAddressSrc, certificate_src::CertificateSrc};

use super::server_behaviors::ServerBehavior;

pub trait NetServer {
    fn start<TServerBehavior: ServerBehavior>(
        src: &impl EndpointAddressSrc,
    ) -> impl Future<Output = Result<(), anyhow::Error>>;
}

pub trait SecureNetServer {
    fn start<TServerBehavior: ServerBehavior>(
        endpoint_src: &impl EndpointAddressSrc,
        certificate_src: &impl CertificateSrc,
    ) -> impl Future<Output = Result<(), anyhow::Error>>;
}
