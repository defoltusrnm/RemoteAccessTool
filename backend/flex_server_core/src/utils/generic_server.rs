use std::marker::PhantomData;

use flex_net_core::networking::address_src::EndpointAddressSrc;

use crate::networking::{
    listeners::NetListener, server_behaviors::ServerBehavior, servers::NetServer,
};

pub struct GenericServer<TListener: NetListener> {
    listener: PhantomData<TListener>,
}

impl<TListener: NetListener> NetServer for GenericServer<TListener> {
    async fn start<TServerBehavior: ServerBehavior>(
        src: &impl EndpointAddressSrc,
    ) -> Result<(), anyhow::Error> {
        let addr = src.get()?;
        log::info!("server will try to use {0}:{1}", addr.host, addr.port);

        let acceptable = TListener::bind(addr).await?;
        TServerBehavior::handle(acceptable).await?;

        Ok(())
    }
}
