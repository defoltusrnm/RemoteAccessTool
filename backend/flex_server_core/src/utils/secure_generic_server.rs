use std::marker::PhantomData;

use flex_net_core::networking::{address_src::EndpointAddressSrc, certificate_src::CertificateSrc};

use crate::networking::{
    listeners::{NetAcceptable, SecureNetListener},
    server_behaviors::ServerBehavior,
    servers::SecureNetServer,
};

pub struct SecureGenericServer<TListener: SecureNetListener + NetAcceptable> {
    listener: PhantomData<TListener>,
}

impl<TListener> SecureNetServer for SecureGenericServer<TListener>
where
    TListener: SecureNetListener + NetAcceptable,
{
    async fn start<TServerBehavior: ServerBehavior>(
        endpoint_src: &impl EndpointAddressSrc,
        certificate_src: &impl CertificateSrc,
    ) -> Result<(), anyhow::Error> {
        let endpoint = endpoint_src.get()?;

        log::info!(
            "server will try to use {0}:{1}",
            endpoint.host,
            endpoint.port
        );

        let certificate = certificate_src.get().await?;
        let acceptable = TListener::bind(endpoint, certificate).await?;
        log::info!("server ready to receive new connections");

        TServerBehavior::handle(acceptable).await?;

        Ok(())
    }
}
