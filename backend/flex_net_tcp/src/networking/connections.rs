use anyhow::Context;
use flex_net_core::networking::{
    connections::{NetConnection, NetReader, NetWriter},
    messages::NetMessage,
};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpStream,
};

pub struct NetTcpConnection {
    inner_socket: TcpStream,
}

impl NetTcpConnection {
    pub fn from_tcp_stream(stream: TcpStream) -> NetTcpConnection {
        NetTcpConnection {
            inner_socket: stream,
        }
    }
}

impl NetConnection for NetTcpConnection {}

impl NetReader for NetTcpConnection {
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

impl NetWriter for NetTcpConnection {
    async fn write(&mut self, buffer: &[u8]) -> Result<(), anyhow::Error> {
        self.inner_socket
            .write(buffer)
            .await
            .with_context(|| "failed to send to connection")
            .map(|_| ())
    }
}
