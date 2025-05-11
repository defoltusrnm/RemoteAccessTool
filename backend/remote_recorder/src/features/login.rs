use anyhow::{Context, bail};
use std::env;

pub async fn check_credentials(login: String, password: String) -> Result<(), anyhow::Error> {
    let expected_login = env::var("LOGIN").with_context(|| "cannot get login")?;
    let expected_password = env::var("PASSWORD").with_context(|| "cannot get login")?;

    if expected_login == login && expected_password == password {
        Ok(())
    } else {
        bail!("fail to login")
    }
}
