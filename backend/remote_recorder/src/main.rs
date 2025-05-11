pub mod utils;

use crate::utils::reading::*;
use flex_net_core::{
    networking::connections::NetConnection, utils::env_host_source::EnvEndpointAddressSrc,
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
use utils::{logger::configure_logs, reading::ReadByte};

type Server = SecureGenericServer<SecureTcpNetListener>;
type ServerBehavior = InfiniteReadBehavior<ProcessRemoteAccessConnection>;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    // let mut set = JoinSet::<()>::new();
    // let monitors = Monitor::all().with_context(|| "failed to get monitors")?;

    // for monitor in monitors {
    //     let (recorder, sx) = monitor
    //         .video_recorder()
    //         .with_context(|| format!("failed to record {:?}", monitor.name()))?;

    //     let stream = sx.into_stream();

    //     set.spawn(
    //         stream.for_each(async |frame| println!("frame: {0}x{1}", frame.width, frame.height)),
    //     );

    //     recorder
    //         .start()
    //         .with_context(|| format!("failed to record {:?}", monitor.name()))?;
    // }
    dotenv::dotenv().ok();
    configure_logs(log::LevelFilter::Trace)?;

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

        match command_frame {
            Command::Login => {
                let login = connection.extract_string().await?;
                let password = connection.extract_string().await?;
            }
        };

        Ok(())
    }
}

enum Command {
    Login,
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
