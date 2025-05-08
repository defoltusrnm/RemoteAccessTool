use derive_more::Display;

#[derive(Display)]
pub struct ServerError {
    message: String
}

impl ServerError {
    pub fn new(message: String) -> ServerError {
        ServerError { message }
    }
}