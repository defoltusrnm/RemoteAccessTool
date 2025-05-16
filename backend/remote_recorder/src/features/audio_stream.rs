use flex_net_core::networking::connections::{LockedWriter, NetConnection, NetWriter};
use futures::StreamExt;

use crate::{media::audio::create_audio_stream, utils::writing::NumberWrite};

use super::{
    events::{Event, WriteEvent},
    protocol_traits::StreamAudioFlow,
};

impl<T: NetConnection> StreamAudioFlow for T {
    async fn stream_audio(&mut self) -> Result<(), anyhow::Error> {
        let mut audio_stream = create_audio_stream()?;

        while let Some(data) = audio_stream.next().await {
            let mut lock = self.lock_write().await;

            lock.write_event(Event::Audio).await?;

            let buff_len: u32 = data.len().try_into()?;
            lock.write_number(buff_len).await?;
            lock.write(&data).await?;

            lock.release();
        }

        Ok(())
    }
}
