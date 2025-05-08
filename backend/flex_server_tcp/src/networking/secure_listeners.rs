use anyhow::Context;
use flex_net_core::networking::{address_src::EndpointAddress, certificate_src::Certificate};
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
    ) -> Result<SecureTcpNetListener, anyhow::Error> {
        let identity = Identity::from_pkcs12(&cert.cert_bytes, &cert.cert_pwd)
            .with_context(|| "Failed to read certificate")
            .and_then(|identity| {
                NativeTlsAcceptor::builder(identity)
                    .build()
                    .with_context(|| "Failed to build secure session")
            })
            .map(TlsAcceptor::from)?;

        let listener = TcpListener::bind(format!("{0}:{1}", addr.host, addr.port))
            .await
            .with_context(|| "Failed to bind")?;

        Ok(SecureTcpNetListener {
            inner_listener: listener,
            acceptor: identity,
        })
    }
}

impl NetAcceptable<SecureNetTcpConnection> for SecureTcpNetListener {
    async fn accept(&self) -> Result<SecureNetTcpConnection, anyhow::Error> {
        let (socket, _) = self
            .inner_listener
            .accept()
            .await
            .with_context(|| "Failed to accept")?;

        let secured_socket = self
            .acceptor
            .accept(socket)
            .await
            .with_context(|| "Failed to establish secure connection")?;

        Ok(SecureNetTcpConnection::from_tcp_stream(secured_socket))
    }
}
