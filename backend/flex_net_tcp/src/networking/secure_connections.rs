use anyhow::Context;
use flex_net_core::networking::{
    connections::{NetConnection, NetReader, NetWriter},
    messages::NetMessage,
};
use tokio::{io::AsyncReadExt, net::TcpStream};
use tokio_native_tls::TlsStream;

pub struct SecureNetTcpConnection {
    inner_socket: TlsStream<TcpStream>,
}

impl SecureNetTcpConnection {
    pub fn from_tcp_stream(stream: TlsStream<TcpStream>) -> SecureNetTcpConnection {
        SecureNetTcpConnection {
            inner_socket: stream,
        }
    }
}

impl NetConnection for SecureNetTcpConnection {}

impl NetReader for SecureNetTcpConnection {
    async fn read(&mut self, buffer_len: usize) -> Result<NetMessage, anyhow::Error> {
        let mut buff = vec![0u8; buffer_len];

        let len = self
            .inner_socket
            .read(&mut buff)
            .await
            .with_context(|| "Cannot read socket")?;
        buff.truncate(len);

        Ok(NetMessage::new(buff))
    }

    async fn read_exactly(&mut self, buffer_len: usize) -> Result<NetMessage, anyhow::Error> {
        let mut buff = vec![0u8; buffer_len];

        _ = self
            .inner_socket
            .read_exact(&mut buff)
            .await
            .with_context(|| "Cannot read exact buffer");

        Ok(NetMessage::new(buff))
    }
}

impl NetWriter for SecureNetTcpConnection {
    fn write(self) {
        todo!()
    }
}
