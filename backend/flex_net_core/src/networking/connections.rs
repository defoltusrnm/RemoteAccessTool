use super::messages::NetMessage;

pub trait NetWriter
where
    Self: Send + Sized,
{
    fn write(self);
}

pub trait NetReader
where
    Self: Send + Sized,
{
    fn read(
        &mut self,
        buffer_len: usize,
    ) -> impl Future<Output = Result<NetMessage, anyhow::Error>>;

    fn read_exactly(
        &mut self,
        buffer_len: usize,
    ) -> impl Future<Output = Result<NetMessage, anyhow::Error>>;
}

pub trait NetConnection: NetReader + NetWriter {}
