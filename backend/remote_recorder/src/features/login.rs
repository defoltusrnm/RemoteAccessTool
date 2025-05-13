use anyhow::{Context, bail};
use flex_net_core::networking::connections::NetConnection;
use futures::TryFutureExt;

use super::{
    commands::{Command, ReadCommand},
    protocol_traits::AuthorizationFlow,
};
use crate::utils::writing::*;
use crate::{features::events::*, utils::reading::*};

impl<T: NetConnection + 'static> AuthorizationFlow for T {
    async fn authorize(&mut self) -> Result<(), anyhow::Error> {
        let command_frame = self.read_command().await?;

        if command_frame == Command::Login {
            let command_id = self.read_number::<u32>().await?;
            log::trace!("got command: {command_id}");

            let login = self.extract_string().await?;
            let password = self.extract_string().await?;

            let result = check_credentials(login, password)
                .inspect_err(|err| log::trace!("failed to authorize: {err}"))
                .await;

            self.write_number(command_id).await?;

            let (status, is_fail) = match result {
                Ok(()) => (Event::Authenticated, false),
                Err(_) => (Event::UnAuthenticated, true),
            };
            self.write_event(status).await?;

            if is_fail {
                bail!("session terminated due to failed login")
            }
        } else {
            self.write_string_with_size(&"иди нахуй долбаеб").await?;
            bail!("expected login flow, but haven't got it")
        }

        Ok(())
    }
}

async fn check_credentials(login: String, password: String) -> Result<(), anyhow::Error> {
    let expected_login = std::env::var("LOGIN").with_context(|| "cannot get login")?;
    let expected_password = std::env::var("PASSWORD").with_context(|| "cannot get login")?;

    if expected_login == login && expected_password == password {
        Ok(())
    } else {
        bail!("fail to login")
    }
}
