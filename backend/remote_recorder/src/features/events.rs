use flex_net_core::networking::connections::NetWriter;

#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub enum Event {
    Authenticated,
    UnAuthenticated,
    Screen,
    Audio,
}

pub trait WriteEvent {
    fn write_event(
        &mut self,
        event: Event,
    ) -> impl Future<Output = Result<(), anyhow::Error>> + Send;
}

impl<T: NetWriter> WriteEvent for T {
    async fn write_event(&mut self, event: Event) -> Result<(), anyhow::Error> {
        let byte = match event {
            Event::Authenticated => 1u8,
            Event::UnAuthenticated => 2u8,
            Event::Screen => 3u8,
            Event::Audio => 4u8,
        };

        self.write(&[byte]).await?;
        Ok(())
    }
}
