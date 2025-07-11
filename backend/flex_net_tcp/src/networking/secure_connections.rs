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
use tokio_native_tls::TlsStream;

pub struct SecureNetTcpConnection {
    inner_socket: TlsStream<TcpStream>,
    inner_write_lock: Arc<Mutex<()>>,
}

impl SecureNetTcpConnection {
    pub fn from_tcp_stream(stream: TlsStream<TcpStream>) -> SecureNetTcpConnection {
        SecureNetTcpConnection {
            inner_socket: stream,
            inner_write_lock: Arc::new(Mutex::new(())),
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

        let read = self
            .inner_socket
            .read_exact(&mut buff)
            .await
            .with_context(|| "Cannot read exact buffer")?;

        log::trace!("requested to read {buffer_len}, got: {read}");

        Ok(NetMessage::new(buff))
    }
}

impl NetWriter for SecureNetTcpConnection {
    async fn write(&mut self, buffer: &[u8]) -> Result<(), anyhow::Error> {
        log::trace!("sending {0} of bytes", buffer.len());

        let mut total_sent = 0;

        while total_sent < buffer.len() {
            let sent = self
                .inner_socket
                .write(&buffer[total_sent..])
                .await
                .with_context(|| "failed to send to connection")?;

            if sent == 0 {
                return Err(anyhow::anyhow!("connection closed while sending data"));
            }

            total_sent += sent;
        }

        Ok(())
    }
}

impl WriterLock for SecureNetTcpConnection {
    async fn lock_write<'a>(&'a mut self) -> impl LockedWriter {
        log::trace!("awaiting to acquire write_lock");
        let lock_fut = self.inner_write_lock.clone().lock_owned().await;
        let locked = ImplLockedWriter::<'a, _> {
            guard: Some(lock_fut),
            inner_write: self,
        };

        log::trace!("transaction started");
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
        log::trace!("transaction ended");
        self.guard.take();
    }
}
