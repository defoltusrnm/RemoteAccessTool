use flex_net_core::networking::connections::NetConnection;

use super::protocol_traits::StreamAudioFlow;

impl<T: NetConnection> StreamAudioFlow for T {
    async fn stream_audio(&mut self) -> Result<(), anyhow::Error> {
        // while let Some(data) = audio_stream.next().await {
        //     let mut lock = self.lock_write().await;

        //     lock.write_event(Event::Audio).await?;

        //     let buff_len: u32 = data.len().try_into()?;
        //     lock.write_number(buff_len).await?;
        //     lock.write(&data).await?;

        //     lock.release();
        // }

        Ok(())
    }
}
