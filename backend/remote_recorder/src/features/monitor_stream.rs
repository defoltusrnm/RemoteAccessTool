use crate::features::events::WriteEvent;
use crate::utils::writing::NumberWrite;
use anyhow::Context;
use flex_net_core::networking::connections::NetConnection;
use futures::StreamExt;
use futures::stream::select_all;
use xcap::Monitor;

use crate::utils::stream::IntoStream;

use super::events::Event;
use super::protocol_traits::StreamMonitorFlow;

impl<T: NetConnection + Send + 'static> StreamMonitorFlow for T {
    async fn stream_screen(&mut self) -> Result<(), anyhow::Error> {
        let monitors = Monitor::all().with_context(|| "failed to get monitors")?;
        let mut streams = Vec::with_capacity(monitors.len());

        for monitor in monitors {
            let (recorder, sx) = monitor
                .video_recorder()
                .with_context(|| format!("failed to record {:?}", monitor.name()))?;

            let id = monitor.id().with_context(|| "failed to get monitor id")?;
            let stream = Box::pin(sx.into_stream().map(move |f| (f, id)));
            streams.push(stream);

            recorder
                .start()
                .with_context(|| format!("failed to record {:?}", monitor.name()))?;
        }

        let mut multiplexer = select_all(streams);
        while let Some((frame, id)) = multiplexer.next().await {
            self.write_event(Event::Screen).await?;
            self.write_number(id).await?;
            self.write_number(frame.raw.len()).await?;
            self.write(&frame.raw).await?;
        }

        Ok(())
    }
}
