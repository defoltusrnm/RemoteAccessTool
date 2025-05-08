use std::pin::Pin;

use flex_net_core::{async_utils::async_and_then::AsyncAndThen, error_handling::server_errors::ServerError, networking::{address_src::EndpointAddressSrc, certificate_src::CertificateSrc, connections::NetConnection}};

use crate::networking::{listeners::{NetAcceptable, SecureNetListener}, servers::SecureNetServer};


pub struct SecureGenericServer;

impl<TConnection, TListener> SecureNetServer<TConnection, TListener> for SecureGenericServer
where
    TConnection: NetConnection,
    TListener: SecureNetListener + NetAcceptable<TConnection> + Send,
{
    async fn start<TEndpointAddrSrc, TCertificateSrc>(
        endpoint_src: TEndpointAddrSrc,
        certificate_src: TCertificateSrc,
        server_handler: Box<
            dyn Fn(TListener) -> Pin<Box<dyn Future<Output = Result<(), ServerError>>>>,
        >,
    ) -> Result<(), ServerError>
    where
        TEndpointAddrSrc: EndpointAddressSrc,
        TCertificateSrc: CertificateSrc
    {
        let endpoint = endpoint_src .get() .inspect(|addr| log::info!("server will try to use {0}:{1}", addr.host, addr.port))?;
        let certificate = certificate_src.get().await?;

        let x = TListener::bind(endpoint, certificate)
            .await
            .inspect(|_| log::info!("server ready to receive new connections"))
            .and_then_async(|listener| server_handler(listener))
            .await;

        x
    }
}
