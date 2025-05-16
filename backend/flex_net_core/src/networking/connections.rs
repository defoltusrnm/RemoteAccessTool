use super::messages::NetMessage;

pub trait NetWriter
where
    Self: Send + Sized,
{
    fn write(&mut self, buffer: &[u8]) -> impl Future<Output = Result<(), anyhow::Error>> + Send;
}

pub trait NetReader
where
    Self: Send + Sized,
{
    fn read(
        &mut self,
        buffer_len: usize,
    ) -> impl Future<Output = Result<NetMessage, anyhow::Error>> + Send;

    fn read_exactly(
        &mut self,
        buffer_len: usize,
    ) -> impl Future<Output = Result<NetMessage, anyhow::Error>> + Send;
}

pub trait NetConnection: NetReader + NetWriter + WriterLock {}

pub trait WriterLock {
    fn lock_write<'a>(&'a mut self) -> impl Future<Output = impl LockedWriter + Send> + Send;
}

pub trait LockedWriter: NetWriter {
    fn release(self);
}
