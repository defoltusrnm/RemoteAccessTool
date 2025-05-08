use flex_net_core::networking::{
    address_src::EndpointAddress, certificate_src::Certificate, connections::NetConnection,
};

pub trait NetAcceptable {
    fn accept(&self) -> impl Future<Output = Result<impl NetConnection + 'static, anyhow::Error>>;
}

pub trait NetListener
where
    Self: Sized,
{
    fn bind(
        addr: EndpointAddress,
    ) -> impl Future<Output = Result<impl NetAcceptable + 'static, anyhow::Error>>;
}

pub trait SecureNetListener
where
    Self: Sized,
{
    fn bind(
        addr: EndpointAddress,
        cert: Certificate,
    ) -> impl Future<Output = Result<impl NetAcceptable + 'static, anyhow::Error>>;
}
