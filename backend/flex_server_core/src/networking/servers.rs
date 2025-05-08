use std::pin::Pin;

use flex_net_core::networking::{
    address_src::EndpointAddressSrc, certificate_src::CertificateSrc, connections::NetConnection,
};

use super::listeners::{NetAcceptable, NetListener, SecureNetListener};

pub trait NetServer<TConnection, TListener>
where
    TConnection: NetConnection,
    TListener: NetListener + NetAcceptable<TConnection>,
{
    fn start(
        src: impl EndpointAddressSrc,
        server_handler: Box<
            dyn Fn(TListener) -> Pin<Box<dyn Future<Output = Result<(), anyhow::Error>>>>,
        >,
    ) -> impl Future<Output = Result<(), anyhow::Error>>;
}

pub trait SecureNetServer<TConnection, TListener>
where
    TConnection: NetConnection,
    TListener: SecureNetListener + NetAcceptable<TConnection>,
{
    fn start(
        endpoint_src: impl EndpointAddressSrc,
        certificate_src: impl CertificateSrc,
        server_handler: Box<
            dyn Fn(TListener) -> Pin<Box<dyn Future<Output = Result<(), anyhow::Error>>>>,
        >,
    ) -> impl Future<Output = Result<(), anyhow::Error>>;
}
