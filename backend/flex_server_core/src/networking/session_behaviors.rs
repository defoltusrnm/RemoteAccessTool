use anyhow::{Context, bail};
use flex_net_core::networking::connections::NetConnection;

pub async fn infinite_read<TConnection>(mut connection: TConnection) -> Result<(), anyhow::Error>
where
    TConnection: NetConnection,
{
    loop {
        let frame = connection.read(512).await?;
        let msg_res = frame
            .to_string()
            .with_context(|| "Cannot get frame as string")
            .and_then(|x| {
                if x.len() > 0 {
                    Ok(x)
                } else {
                    bail!("Empty read")
                }
            })?;

        log::trace!("Received {msg_res}");
    }
}
