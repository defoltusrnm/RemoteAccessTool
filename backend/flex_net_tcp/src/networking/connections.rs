use std::sync::Arc;

use anyhow::Context;
use flex_net_core::networking::{
    connections::{LockedWriter, NetConnection, NetReader, NetWriter, WriterLock},
    messages::NetMessage,
};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpStream,
    sync::{Mutex, OwnedMutexGuard},
};

pub struct NetTcpConnection {
    inner_socket: TcpStream,
    inner_write_lock: Arc<Mutex<()>>,
}

impl NetTcpConnection {
    pub fn from_tcp_stream(stream: TcpStream) -> NetTcpConnection {
        NetTcpConnection {
            inner_socket: stream,
            inner_write_lock: Arc::new(Mutex::new(())),
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

impl WriterLock for NetTcpConnection {
    async fn lock_write<'a>(&'a mut self) -> impl LockedWriter {
        let lock_fut = self.inner_write_lock.clone().lock_owned().await;
        let locked = ImplLockedWriter::<'a, _> {
            guard: Some(lock_fut),
            inner_write: self,
        };

        locked
    }
}

struct ImplLockedWriter<'a, T: NetWriter> {
    guard: Option<OwnedMutexGuard<()>>,
    inner_write: &'a mut T,
}

impl<'a, T: NetWriter> NetWriter for ImplLockedWriter<'a, T> {
    async fn write(&mut self, buffer: &[u8]) -> Result<(), anyhow::Error> {
        self.inner_write.write(buffer).await
    }
}

impl<'a, T: NetWriter> LockedWriter for ImplLockedWriter<'a, T> {
    fn release(mut self) {
        self.guard.take();
    }
}
