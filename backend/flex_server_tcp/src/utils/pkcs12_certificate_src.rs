use std::env;

use anyhow::Context;
use flex_net_core::{
    async_utils::async_and_then::AsyncAndThen,
    networking::certificate_src::{Certificate, CertificateSrc},
};
use tokio::{fs::File, io::AsyncReadExt};

pub struct Pkcs12CertificateSrc {
    cert_path_env: String,
    cert_pwd_env: String,
}

impl Pkcs12CertificateSrc {
    pub fn new_from_env(file_name: &str, env_name: &str) -> Self {
        Pkcs12CertificateSrc {
            cert_path_env: file_name.to_owned(),
            cert_pwd_env: env_name.to_owned(),
        }
    }
}

impl CertificateSrc for Pkcs12CertificateSrc {
    async fn get(&self) -> Result<Certificate, anyhow::Error> {
        let pwd_env_result =
            env::var(&self.cert_pwd_env).with_context(|| "Failed to get certificate pwd")?;

        let cert_path =
            env::var(&self.cert_path_env).with_context(|| "Failed to get certificate path")?;

        let cert_content = File::open(cert_path)
            .await
            .with_context(|| "Failed read certificate content")
            .and_then_async(async |mut f: File| {
                let mut content = vec![];
                _ = f
                    .read_to_end(&mut content)
                    .await
                    .inspect_err(|err| log::error!("read error {err}"));
                Ok(content)
            })
            .await?;

        Ok(Certificate {
            cert_bytes: cert_content,
            cert_pwd: pwd_env_result,
        })
    }
}
