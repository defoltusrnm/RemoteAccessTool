use anyhow::{Context, bail};
use flex_net_core::networking::connections::NetReader;

use super::numbers::EndianRead;

pub trait ReadByte {
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

pub trait ReadInteger {
    fn read_number<Number: EndianRead>(
        &mut self,
    ) -> impl Future<Output = Result<Number, anyhow::Error>>;
}

impl<T: ReadByte + NetReader> ReadInteger for T {
    async fn read_number<Number: EndianRead>(&mut self) -> Result<Number, anyhow::Error> {
        let endianness_byte = self.read_single_byte().await?;
        let number_frame = self.read_exactly(Number::size()).await?;

        let number_parser = match endianness_byte {
            0 => |b: Number::Array| Result::<Number, anyhow::Error>::Ok(Number::from_be_bytes(b)),
            1 => |b: Number::Array| Ok(Number::from_le_bytes(b)),
            _ => bail!("failed to decide endianness"),
        };

        Number::from_slice(number_frame.bytes()).and_then(number_parser)
    }
}

pub trait ReadString {
    fn read_string(&mut self, len: u32) -> impl Future<Output = Result<String, anyhow::Error>>;
}

impl<T: NetReader> ReadString for T {
    async fn read_string(&mut self, len: u32) -> Result<String, anyhow::Error> {
        let buffer_len: usize = len
            .try_into()
            .with_context(|| "cannot get usize from u32")?;
        let frame = self.read_exactly(buffer_len).await?;
        return frame.to_string();
    }
}

pub trait ExtractString {
    fn extract_string(&mut self) -> impl Future<Output = Result<String, anyhow::Error>>;
}

impl<T: ReadInteger + ReadString> ExtractString for T {
    async fn extract_string(&mut self) -> Result<String, anyhow::Error> {
        let size = self.read_number().await?;
        self.read_string(size).await
    }
}
