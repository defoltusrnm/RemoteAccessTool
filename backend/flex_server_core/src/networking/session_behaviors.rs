use flex_net_core::{
    error_handling::server_errors::ServerError, networking::connections::NetConnection,
};


pub async fn infinite_read<TConnection>(mut connection: TConnection) -> Result<(), ServerError>
where
    TConnection: NetConnection,
{
    loop {
        match connection.read(512).await {
            Ok(frame) => {
                let msg_res = frame.to_string().and_then(|x| {
                    Some(x)
                        .filter(|y| y.len() > 0)
                        .map(|y| Ok(y))
                        .unwrap_or_else(|| Err(ServerError::new(format!("empty read"))))
                });

                match msg_res {
                    Ok(msg) => log::info!("received messages {msg}"),
                    Err(err) => return Err(err),
                }
            }
            Err(err) => {
                return Err(err);
            }
        };
    }
}
