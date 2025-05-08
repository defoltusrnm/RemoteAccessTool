use std::pin::Pin;

use flex_net_core::{
    async_utils::async_and_then::AsyncAndThen,
    networking::{address_src::EndpointAddressSrc, connections::NetConnection},
};

use crate::networking::{
    listeners::{NetAcceptable, NetListener},
    servers::NetServer,
};

pub struct GenericServer;

impl<TConnection, TListener> NetServer<TConnection, TListener> for GenericServer
where
    TConnection: NetConnection,
    TListener: NetListener + Send + NetAcceptable<TConnection>,
{
    async fn start<TEndpointAddrSrc>(
        src: TEndpointAddrSrc,
        server_handler: Box<
            dyn Fn(TListener) -> Pin<Box<dyn Future<Output = Result<(), anyhow::Error>>>>,
        >,
    ) -> Result<(), anyhow::Error>
    where
        TEndpointAddrSrc: EndpointAddressSrc,
    {
        let x = src
            .get()
            .inspect(|addr| log::info!("server will try to use {0}:{1}", addr.host, addr.port))
            .and_then_async(|addr| TListener::bind(addr))
            .await
            .inspect(|_| log::info!("server ready to receive new connections"))
            .and_then_async(|listener| server_handler(listener))
            .await;

        x
    }
}
