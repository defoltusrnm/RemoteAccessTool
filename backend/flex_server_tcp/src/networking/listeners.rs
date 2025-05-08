use anyhow::Context;
use flex_net_core::networking::{address_src::EndpointAddress, connections::NetConnection};
use flex_net_tcp::networking::connections::NetTcpConnection;
use flex_server_core::networking::listeners::{NetAcceptable, NetListener};
use tokio::net::TcpListener;

pub struct NetTcpListener {
    inner_listener: TcpListener,
}

impl NetListener for NetTcpListener {
    async fn bind(addr: EndpointAddress) -> Result<impl NetAcceptable, anyhow::Error> {
        let listener = TcpListener::bind(format!("{0}:{1}", addr.host, addr.port))
            .await
            .with_context(|| "Cannot bind to remote")?;

        Ok(NetTcpListener {
            inner_listener: listener,
        })
    }
}

impl NetAcceptable for NetTcpListener {
    async fn accept(&self) -> Result<impl NetConnection + 'static, anyhow::Error> {
        let (socket, _) = self
            .inner_listener
            .accept()
            .await
            .with_context(|| "Failed to get new connection")?;

        Ok(NetTcpConnection::from_tcp_stream(socket))
    }
}
