use flex_net_core::{
    error_handling::server_errors::ServerError,
    networking::{
        connections::{NetConnection, NetReader, NetWriter},
        messages::NetMessage,
    },
};
use tokio::{io::AsyncReadExt, net::TcpStream};

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
    async fn read(&mut self, buffer_len: usize) -> Result<NetMessage, ServerError> {
        let mut buff = vec![0u8; buffer_len];

        match self.inner_socket.read(&mut buff).await {
            Ok(len) => {
                buff.truncate(len);
                Ok(NetMessage::new(buff))
            }
            Err(err) => Err(ServerErrors::buffer_read_error(err)),
        }
    }

    async fn read_exactly(&mut self, buffer_len: usize) -> Result<NetMessage, ServerError> {
        let mut buff = vec![0u8; buffer_len];

        _ = self
            .inner_socket
            .read_exact(&mut buff)
            .await
            .map_err(ServerErrors::buffer_read_error)?;

        Ok(NetMessage::new(buff))
    }
}

impl NetWriter for NetTcpConnection {
    fn write(self) {
        todo!()
    }
}

struct ServerErrors;

impl ServerErrors {
    pub fn buffer_read_error(err: std::io::Error) -> ServerError {
        ServerError::new(format!("error when read from connection: {err}"))
    }
}
