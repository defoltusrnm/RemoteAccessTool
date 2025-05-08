mod app_logging;

use app_logging::logger_cfg::configure_logs;
use dotenv::dotenv;
use flex_net_core::{
    error_handling::server_errors::ServerError, networking::connections::NetConnection,
    utils::env_host_source::EnvEndpointAddressSrc,
};
use flex_net_tcp::networking::secure_connections::SecureNetTcpConnection;
use flex_server_core::{
    networking::{server_behaviors, servers::SecureNetServer},
    utils::secure_generic_server::SecureGenericServer,
};
use flex_server_tcp::{
    networking::secure_listeners::SecureTcpNetListener,
    utils::pkcs12_certificate_src::Pkcs12CertificateSrc,
};
use log::LevelFilter;

#[tokio::main]
async fn main() {
    configure_logs(LevelFilter::Trace).unwrap();

    match dotenv() {
        Ok(_) => log::trace!(".env loaded"),
        Err(err) => log::trace!(".env failed to load due to {err}"),
    };

    secure_server().await;
}

async fn secure_server() {
    let server_handler =
        server_behaviors::infinite_read::<SecureNetTcpConnection, SecureTcpNetListener, _, _>(
            &exact_read::<SecureNetTcpConnection>,
        );

    match SecureGenericServer::start(
        EnvEndpointAddressSrc::new_with_port_fallback(4141),
        Pkcs12CertificateSrc::new_from_env("CERT_PATH", "CERT_PWD"),
        server_handler,
    )
    .await
    {
        Ok(()) => log::info!("server ended it's work"),
        Err(err) => log::error!("server ended it's work with: {err}"),
    }
}

pub async fn exact_read<TConnection>(mut connection: TConnection) -> Result<(), ServerError>
where
    TConnection: NetConnection,
{
    loop {
        let msg_size = connection.read_exactly(8).await.map(|msg| {
            let mut usize_bytes = [0u8; 8];
            usize_bytes.copy_from_slice(&msg.bytes());

            if cfg!(target_endian = "big") {
                usize::from_be_bytes(usize_bytes)
            } else {
                usize::from_le_bytes(usize_bytes)
            }
        })?;

        let actual_message = connection.read_exactly(msg_size).await?;
        let msg = actual_message.to_string()?;
        
        log::info!("Got message {0} {1} {2}", msg_size, msg.len(), msg);
    }
}
