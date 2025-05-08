use std::pin::Pin;

use flex_net_core::{
    async_utils::async_and_then::AsyncAndThen,
    networking::{
        address_src::EndpointAddressSrc, certificate_src::CertificateSrc,
        connections::NetConnection,
    },
};

use crate::networking::{
    listeners::{NetAcceptable, SecureNetListener},
    servers::SecureNetServer,
};

pub struct SecureGenericServer;

impl<TConnection, TListener> SecureNetServer<TConnection, TListener> for SecureGenericServer
where
    TConnection: NetConnection,
    TListener: SecureNetListener + NetAcceptable<TConnection> + Send,
{
    async fn start(
        endpoint_src: impl EndpointAddressSrc,
        certificate_src: impl CertificateSrc,
        server_handler: Box<
            dyn Fn(TListener) -> Pin<Box<dyn Future<Output = Result<(), anyhow::Error>>>>,
        >,
    ) -> Result<(), anyhow::Error> {
        let endpoint = endpoint_src
            .get()
            .inspect(|addr| log::info!("server will try to use {0}:{1}", addr.host, addr.port))?;
        let certificate = certificate_src.get().await?;

        let x = TListener::bind(endpoint, certificate)
            .await
            .inspect(|_| log::info!("server ready to receive new connections"))
            .and_then_async(|listener| server_handler(listener))
            .await;

        x
    }
}
