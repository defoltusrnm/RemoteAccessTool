pub mod features;
pub mod utils;

use features::protocol_traits::AuthorizationFlow;
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
use utils::logger::configure_logs;

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
        connection.authorize().await?;

        Ok(())
    }
}
