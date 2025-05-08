use std::marker::PhantomData;

use flex_net_core::networking::connections::NetConnection;
use futures::{TryFutureExt, future::ready};
use tokio::task;

use super::listeners::NetAcceptable;

pub trait ServerBehavior {
    fn handle(
        listener: impl NetAcceptable + 'static,
    ) -> impl Future<Output = Result<(), anyhow::Error>>;
}

pub trait ConnectionHandler {
    fn handle(
        connection: impl NetConnection,
    ) -> impl Future<Output = Result<(), anyhow::Error>> + Send + 'static;
}

pub struct EmptyConnectionHandler {}

impl ConnectionHandler for EmptyConnectionHandler {
    fn handle(
        _connection: impl NetConnection,
    ) -> impl Future<Output = Result<(), anyhow::Error>> + Send + 'static {
        ready(Ok(()))
    }
}

pub struct InfiniteReadBehavior<TConnectionHandler: ConnectionHandler> {
    connection_handler: PhantomData<TConnectionHandler>,
}

impl<TConnectionHandler> InfiniteReadBehavior<TConnectionHandler>
where
    TConnectionHandler: ConnectionHandler + Send + 'static,
{
    fn process_connection(
        connection: impl NetConnection + 'static,
    ) -> impl Future<Output = ()> + Send + 'static {
        async move {
            _ = TConnectionHandler::handle(connection)
                .inspect_ok(|_| log::trace!("Connection ended"))
                .inspect_err(|err| log::error!("Connection ended with error: {err}"))
                .await;
        }
    }
}

impl<TConnectionHandler> ServerBehavior for InfiniteReadBehavior<TConnectionHandler>
where
    TConnectionHandler: ConnectionHandler + Send + 'static,
{
    async fn handle(listener: impl NetAcceptable + 'static) -> Result<(), anyhow::Error> {
        let mut set = task::JoinSet::<()>::new();
        loop {
            log::trace!("waiting for new connections");

            let connection = listener.accept().await?;
            log::trace!("got connection");

            set.spawn(Self::process_connection(connection));
        }
    }
}
