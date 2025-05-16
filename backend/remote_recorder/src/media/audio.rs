use anyhow::Context as ctx;
use futures::stream::{RepeatWith, repeat_with};
use libpulse_binding as pulse;
use pulse::context::Context;
use pulse::context::FlagSet;
use pulse::mainloop::standard::Mainloop;
use pulse::proplist::Proplist;
use pulse::stream::PeekResult;
use pulse::stream::Stream;
use std::sync::Arc;
use std::sync::Mutex;
use std::thread;
use std::time::Duration;

const SAMPLE_RATE: u32 = 44100;
const CHANNELS: u8 = 2;

#[cfg(target_os = "linux")]
pub fn create_audio_stream() -> Result<RepeatWith<impl FnMut() -> Vec<u8>>, anyhow::Error> {
    use std::sync::Arc;

    use futures::lock::Mutex;

    let mut mainloop = Mainloop::new().with_context(|| "fail main loop")?;
    let mut proplist = Proplist::new().with_context(|| "failed prop list")?;
    proplist
        .set_str(
            pulse::proplist::properties::APPLICATION_NAME,
            "RustAudioSender",
        )
        .map_err(|_| anyhow::anyhow!("failed to set app name"))?;

    let mut context = Context::new_with_proplist(&mainloop, "RustContext", &proplist)
        .with_context(|| "failed context")?;

    context.connect(None, FlagSet::empty(), None)?;
    mainloop.run().map_err(|_| anyhow::anyhow!("run err"))?;

    while context.get_state() != pulse::context::State::Ready {
        mainloop.iterate(false);
        thread::sleep(Duration::from_millis(10));
    }

    let spec = pulse::sample::Spec {
        format: pulse::sample::Format::S16le,
        channels: CHANNELS,
        rate: SAMPLE_RATE,
    };

    let mut stream =
        Stream::new(&mut context, "Record", &spec, None).with_context(|| "create stream error")?;
    stream.connect_record(None, None, pulse::stream::FlagSet::START_CORKED)?;
    stream.cork(None);

    let src = AudioSrc {
        mainloop: Arc::new(std::sync::Mutex::new(mainloop)),
        stream: Arc::new(std::sync::Mutex::new(stream)),
    };
    let s = repeat_with(move || src.clone().get_audio());

    Ok(s)
}

#[derive(Clone)]
struct AudioSrc {
    mainloop: Arc<Mutex<Mainloop>>,
    stream: Arc<Mutex<Stream>>,
}

impl AudioSrc {
    fn get_audio(self) -> Vec<u8> {
        loop {
            let mut mainloop = self.mainloop.lock().unwrap();
            mainloop.iterate(false);
            let mut stream = self.stream.lock().unwrap();

            if let Ok(PeekResult::Data(data)) = stream.peek() {
                let buf = data.to_vec();
                break buf;
            }
        }
    }
}

unsafe impl Send for AudioSrc {}
