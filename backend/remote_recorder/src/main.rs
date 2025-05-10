pub mod utils;

use anyhow::Context;
use flex_net_core::{
    networking::{
        connections::{self, NetConnection, NetReader},
        messages::NetMessage,
    },
    utils::env_host_source::EnvEndpointAddressSrc,
};
use flex_server_core::{
    networking::{
        server_behaviors::{ConnectionHandler, InfiniteReadBehavior},
        servers::SecureNetServer,
    },
    utils::secure_generic_server::SecureGenericServer,
};
use flex_server_tcp::{
    networking::secure_listeners::SecureTcpNetListener,
    utils::pkcs12_certificate_src::Pkcs12CertificateSrc,
};
use futures::StreamExt;
use tokio::task::JoinSet;
use utils::stream::IntoStream;
use xcap::Monitor;

type Server = SecureGenericServer<SecureTcpNetListener>;
type ServerBehavior = InfiniteReadBehavior<ProcessRemoteAccessConnection>;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let mut set = JoinSet::<()>::new();
    let monitors = Monitor::all().with_context(|| "failed to get monitors")?;

    for monitor in monitors {
        let (recorder, sx) = monitor
            .video_recorder()
            .with_context(|| format!("failed to record {:?}", monitor.name()))?;

        let stream = sx.into_stream();

        set.spawn(
            stream.for_each(async |frame| println!("frame: {0}x{1}", frame.width, frame.height)),
        );

        recorder
            .start()
            .with_context(|| format!("failed to record {:?}", monitor.name()))?;
    }

    Server::start::<ServerBehavior>(
        &EnvEndpointAddressSrc::new_with_port_fallback(4141),
        &Pkcs12CertificateSrc::new_from_env("CERT_PATH", "CERT_PWD"),
    )
    .await
}

struct ProcessRemoteAccessConnection;

impl ConnectionHandler for ProcessRemoteAccessConnection {
    async fn handle(mut connection: impl NetConnection + 'static) -> Result<(), anyhow::Error> {
        let command_frame = connection.read_command().await?;

        Ok(())
    }
}

enum Command {
    Login,
}

trait ReadByte {
    fn read_single_byte(&mut self) -> impl Future<Output = Result<u8, anyhow::Error>> + Send;
}

impl<T: NetReader> ReadByte for T {
    async fn read_single_byte(&mut self) -> Result<u8, anyhow::Error> {
        let frame = self.read_exactly(1).await?;

        let byte = *frame
            .bytes()
            .get(0)
            .with_context(|| "read buffer was empty, but expected to have single element")?;

        Ok(byte)
    }
}

trait ReadCommand {
    fn read_command(&mut self) -> impl Future<Output = Result<Command, anyhow::Error>> + Send;
}

impl<T: ReadByte + Send> ReadCommand for T {
    async fn read_command(&mut self) -> Result<Command, anyhow::Error> {
        let command_byte = self.read_single_byte().await?;

        match command_byte {
            1 => Ok(Command::Login),
            _ => anyhow::bail!("unknown command"),
        }
    }
}

trait ReadInteger {
    fn read_integer(&mut self) -> impl Future<Output = Result<i32, anyhow::Error>>;
}

impl<T: ReadByte + NetReader> ReadInteger for T {
    async fn read_integer(&mut self) -> Result<i32, anyhow::Error> {
        let endianess_byte = self.read_single_byte().await?;
        let number_frame = self.read_exactly(4).await?;

            
    }
}
