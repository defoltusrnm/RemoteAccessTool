pub trait AuthorizationFlow {
    fn authorize(&mut self) -> impl Future<Output = Result<(), anyhow::Error>> + Send;
}

pub trait StreamMonitorFlow {
    fn stream_screen(&mut self) -> impl Future<Output = Result<(), anyhow::Error>> + Send;
}

pub trait StreamAudioFlow {
    fn stream_audio(&mut self) -> impl Future<Output = Result<(), anyhow::Error>> + Send;
}
