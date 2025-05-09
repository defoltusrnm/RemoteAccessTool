use std::{thread, time::Duration};

use anyhow::Context;
use futures::StreamExt;
use tokio::{
    task::{self, JoinSet},
    time::sleep,
};
use xcap::Monitor;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    // let mut set = JoinSet::<()>::new();
    let monitors = Monitor::all().with_context(|| "failed to get monitors")?;

    for monitor in monitors {
        let (recorder, sx) = monitor
            .video_recorder()
            .with_context(|| format!("failed to record {:?}", monitor.name()))?;

        thread::spawn(move || {
            loop {
                match sx.recv() {
                    Ok(frame) => {
                        println!("frame: {0}x{1}", frame.width, frame.height);
                    }
                    _ => continue,
                }
            }
        });

        recorder
            .start()
            .with_context(|| format!("failed to record {:?}", monitor.name()))?;
    }

    sleep(Duration::from_secs(10)).await;

    Ok(())
}
