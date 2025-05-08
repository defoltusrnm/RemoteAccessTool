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
    fn start<TEndpointAddrSrc>(
        src: TEndpointAddrSrc,
        server_handler: Box<
            dyn Fn(TListener) -> Pin<Box<dyn Future<Output = Result<(), anyhow::Error>>>>,
        >,
    ) -> impl Future<Output = Result<(), anyhow::Error>>
    where
        TEndpointAddrSrc: EndpointAddressSrc;
}

pub trait SecureNetServer<TConnection, TListener>
where
    TConnection: NetConnection,
    TListener: SecureNetListener + NetAcceptable<TConnection>,
{
    fn start<TEndpointAddrSrc, TCertificateSrc>(
        endpoint_src: TEndpointAddrSrc,
        certificate_src: TCertificateSrc,
        server_handler: Box<
            dyn Fn(TListener) -> Pin<Box<dyn Future<Output = Result<(), anyhow::Error>>>>,
        >,
    ) -> impl Future<Output = Result<(), anyhow::Error>>
    where
        TEndpointAddrSrc: EndpointAddressSrc,
        TCertificateSrc: CertificateSrc;
}
