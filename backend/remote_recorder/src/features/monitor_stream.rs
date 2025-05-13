use std::time::Duration;
use stream_throttle::{ThrottlePool, ThrottleRate, ThrottledStream};

use crate::features::events::WriteEvent;
use crate::utils::writing::NumberWrite;
use anyhow::{Context, bail};
use flex_net_core::networking::connections::NetConnection;
use futures::StreamExt;
use futures::stream::{repeat_with, select_all};
use xcap::Monitor;

use super::events::Event;
use super::protocol_traits::StreamMonitorFlow;

impl<T: NetConnection + Send + 'static> StreamMonitorFlow for T {
    async fn stream_screen(&mut self) -> Result<(), anyhow::Error> {
        log::trace!("starting to stream available monitors");

        let monitors = Monitor::all().with_context(|| "failed to get monitors")?;
        let mut streams = Vec::with_capacity(monitors.len());

        for monitor in monitors {
            let id = monitor.id().with_context(|| "failed to get monitor id")?;
            let name = monitor.name().with_context(|| "failed to get monitor id")?;

            log::trace!("monitor: {id} {name} prepared");

            let stream = repeat_with(get_capture_func(monitor, id));

            streams.push(stream);
        }

        let rate = ThrottleRate::new(5, Duration::from_millis(16));
        let pool = ThrottlePool::new(rate);

        let mut multiplexer = select_all(streams).throttle(pool);

        while let Some(Ok((frame, id))) = multiplexer.next().await {
            self.write_event(Event::Screen).await?;
            self.write_number(id).await?;
            self.write_number(frame.width).await?;
            self.write_number(frame.height).await?;

            let frame_len: u32 = frame.raw.len().try_into()?;
            self.write_number(frame_len).await?;

            self.write(frame.raw.as_slice()).await?;

            log::trace!("data send");
        }

        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct Frame {
    pub width: u32,
    pub height: u32,
    pub raw: Vec<u8>,
}

impl Frame {
    pub fn new(width: u32, height: u32, raw: Vec<u8>) -> Self {
        Frame { width, height, raw }
    }
}

#[cfg(target_os = "linux")]
fn get_capture_func(monitor: Monitor, id: u32) -> impl Fn() -> Result<(Frame, u32), anyhow::Error> {
    move || {
        let id = monitor.id()?;
        let frame = monitor.capture_image().map(|x| {
            let width = x.width();
            let height = x.height();
            let raw = x.into_raw();

            (Frame::new(width, height, raw), id)
        })?;

        Ok(frame)
    }
}

#[cfg(target_os = "windows")]
fn get_capture_func(
    _monitor: Monitor,
    id: u32,
) -> impl Fn() -> Result<(Frame, u32), anyhow::Error> {
    move || {
        let monitor = get_monitor(id)?;
        let frame = monitor.capture_image().map(|x| {
            let width = x.width();
            let height = x.height();
            let raw = x.into_raw();

            (Frame::new(width, height, raw), id)
        })?;

        Ok(frame)
    }
}

#[cfg(target_os = "windows")]
fn get_monitor(id: u32) -> Result<Monitor, anyhow::Error> {
    let monitors = Monitor::all()?;
    for m in monitors {
        if m.id()? == id {
            return Ok(m);
        }
    }

    bail!("can't get")
}
