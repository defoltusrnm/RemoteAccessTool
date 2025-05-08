use flex_net_core::{
    error_handling::server_errors::ServerError, networking::address_src::EndpointAddress,
};
use flex_net_tcp::networking::connections::NetTcpConnection;
use flex_server_core::networking::listeners::{NetAcceptable, NetListener};
use tokio::net::TcpListener;

pub struct NetTcpListener {
    inner_listener: TcpListener,
}

impl NetListener for NetTcpListener {
    async fn bind(addr: EndpointAddress) -> Result<Self, ServerError> {
        match TcpListener::bind(format!("{0}:{1}", addr.host, addr.port)).await {
            Ok(listener) => Ok(NetTcpListener {
                inner_listener: listener,
            }),
            Err(err) => Err(ServerErrors::bind_error(err)),
        }
    }
}

impl NetAcceptable<NetTcpConnection> for NetTcpListener {
    async fn accept(&self) -> Result<NetTcpConnection, ServerError> {
        match self.inner_listener.accept().await {
            Ok((socket, _)) => Ok(NetTcpConnection::from_tcp_stream(socket)),
            Err(err) => Err(ServerErrors::receive_error(err)),
        }
    }
}

struct ServerErrors;

impl ServerErrors {
    pub fn bind_error(err: std::io::Error) -> ServerError {
        ServerError::new(format!("cannot start server because: {err}"))
    }

    pub fn receive_error(err: std::io::Error) -> ServerError {
        ServerError::new(format!(
            "error when server tried to accept connection: {err}"
        ))
    }
}
