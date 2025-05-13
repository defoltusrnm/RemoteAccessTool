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
        log::trace!("starting to stream available monitors");

        let monitors = Monitor::all().with_context(|| "failed to get monitors")?;
        let mut streams = Vec::with_capacity(monitors.len());
        let mut recorders = Vec::with_capacity(monitors.len());

        for monitor in monitors {
            let (recorder, sx) = monitor
                .video_recorder()
                .with_context(|| format!("failed to record {:?}", monitor.name()))?;

            let id = monitor.id().with_context(|| "failed to get monitor id")?;
            let stream = sx.into_stream().map(move |f| (f, id));

            recorder
                .start()
                .with_context(|| format!("failed to record {:?}", monitor.name()))?;

            log::trace!("monitor: {0} {1} prepared", monitor.id()?, monitor.name()?);

            streams.push(stream);
            recorders.push(recorder);
        }

        let loop_fn = async move || {
            let mut multiplexer = select_all(streams);
            while let Some((frame, id)) = multiplexer.next().await {
                self.write_event(Event::Screen).await?;
                self.write_number(id).await?;
                self.write_number(frame.width).await?;
                self.write_number(frame.height).await?;

                let frame_len: u32 = frame.raw.len().try_into()?;
                self.write_number(frame_len).await?;
                self.write(frame.raw.as_slice()).await?;

                log::trace!("data send");
                // why lag?
                tokio::time::sleep(std::time::Duration::from_secs(3)).await;
                // break;
            }

            Result::<(), anyhow::Error>::Ok(())
        };

        let loop_result = loop_fn().await;
        for rec in recorders {
            rec.stop().with_context(|| "failed to stop recorder")?;
        }
        loop_result?;

        Ok(())
    }
}
