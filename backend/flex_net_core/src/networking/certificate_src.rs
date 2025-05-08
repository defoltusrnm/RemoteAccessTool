use crate::error_handling::server_errors::ServerError;


pub struct Certificate {
    pub cert_bytes: Vec<u8>,
    pub cert_pwd: String
}

pub trait CertificateSrc {
    fn get(&self) -> impl Future<Output = Result<Certificate, ServerError>>;
}