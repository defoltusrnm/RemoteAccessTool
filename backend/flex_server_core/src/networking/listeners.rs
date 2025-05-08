use flex_net_core::{
    error_handling::server_errors::ServerError,
    networking::{
        address_src::EndpointAddress, certificate_src::Certificate, connections::NetConnection,
    },
};

pub trait NetAcceptable<TConnection>
where
    TConnection: NetConnection,
{
    fn accept(&self) -> impl Future<Output = Result<TConnection, ServerError>>;
}

pub trait NetListener
where
    Self: Sized,
{
    fn bind(addr: EndpointAddress) -> impl Future<Output = Result<Self, ServerError>>;
}

pub trait SecureNetListener
where
    Self: Sized,
{
    fn bind(
        addr: EndpointAddress,
        cert: Certificate,
    ) -> impl Future<Output = Result<Self, ServerError>>;
}
