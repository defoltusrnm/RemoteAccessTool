use std::fmt::Display;

use anyhow::Context;
use flex_net_core::networking::connections::NetWriter;

use super::numbers::EndianRead;

pub trait NumberWrite {
    fn write_number<Number: EndianRead + Send>(
        &mut self,
        num: Number,
    ) -> impl Future<Output = Result<(), anyhow::Error>> + Send;
}

impl<T: NetWriter> NumberWrite for T {
    async fn write_number<Number: EndianRead + Send>(
        &mut self,
        num: Number,
    ) -> Result<(), anyhow::Error> {
        let (endian, number_bytes) = if cfg!(target_endian = "big") {
            (0u8, num.be_bytes())
        } else {
            (1u8, num.le_bytes())
        };

        self.write(&[endian]).await?;
        self.write(number_bytes.as_slice()).await?;

        Ok(())
    }
}

pub trait StringSizedWrite {
    fn write_string_with_size(
        &mut self,
        msg: &(impl Display + Send + Sync),
    ) -> impl Future<Output = Result<(), anyhow::Error>> + Send;
}

impl<T: NumberWrite + NetWriter> StringSizedWrite for T {
    async fn write_string_with_size(
        &mut self,
        msg: &(impl Display + Send + Sync),
    ) -> Result<(), anyhow::Error> {
        let msg2 = msg.to_string();

        let l: u32 = msg2.len().try_into().with_context(|| "usize -> u32 fail")?;

        self.write_number(l).await?;
        self.write(msg2.as_bytes()).await?;

        Ok(())
    }
}
