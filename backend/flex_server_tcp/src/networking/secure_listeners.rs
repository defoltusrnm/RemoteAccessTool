use flex_net_core::{
    error_handling::server_errors::ServerError,
    networking::{address_src::EndpointAddress, certificate_src::Certificate},
};
use flex_net_tcp::networking::secure_connections::SecureNetTcpConnection;
use flex_server_core::networking::listeners::{NetAcceptable, SecureNetListener};
use native_tls::{Identity, TlsAcceptor as NativeTlsAcceptor};
use tokio::net::TcpListener;
use tokio_native_tls::TlsAcceptor;

pub struct SecureTcpNetListener {
    inner_listener: TcpListener,
    acceptor: TlsAcceptor,
}

impl SecureNetListener for SecureTcpNetListener {
    async fn bind(
        addr: EndpointAddress,
        cert: Certificate,
    ) -> Result<SecureTcpNetListener, ServerError> {
        let identity = Identity::from_pkcs12(&cert.cert_bytes, &cert.cert_pwd)
            .map_err(ServerErrors::cannot_read_cert)
            .and_then(|identity| {
                NativeTlsAcceptor::builder(identity)
                    .build()
                    .map_err(ServerErrors::cannot_read_cert)
            })
            .map(TlsAcceptor::from)?;

        match TcpListener::bind(format!("{0}:{1}", addr.host, addr.port)).await {
            Ok(listener) => Ok(SecureTcpNetListener {
                inner_listener: listener,
                acceptor: identity,
            }),
            Err(err) => Err(ServerErrors::bind_error(err)),
        }
    }
}

impl NetAcceptable<SecureNetTcpConnection> for SecureTcpNetListener {
    async fn accept(&self) -> Result<SecureNetTcpConnection, ServerError> {
        let (socket, _) = self
            .inner_listener
            .accept()
            .await
            .map_err(ServerErrors::receive_error)?;

        let secured_socket = self
            .acceptor
            .accept(socket)
            .await
            .map_err(ServerErrors::tls_error)?;

        Ok(SecureNetTcpConnection::from_tcp_stream(secured_socket))
    }
}

struct ServerErrors;

impl ServerErrors {
    pub fn cannot_read_cert(err: native_tls::Error) -> ServerError {
        ServerError::new(format!("cannot read cert: {err}"))
    }

    pub fn bind_error(err: std::io::Error) -> ServerError {
        ServerError::new(format!("cannot start server because: {err}"))
    }

    pub fn receive_error(err: std::io::Error) -> ServerError {
        ServerError::new(format!(
            "error when server tried to accept connection: {err}"
        ))
    }

    pub fn tls_error(err: native_tls::Error) -> ServerError {
        ServerError::new(format!(
            "error when server tried to secure connection: {err}"
        ))
    }
}
