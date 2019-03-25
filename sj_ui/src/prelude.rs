pub use actix::*;
pub use futures::future::{self, Either, Future};
pub use sj_token::SJAccess;

pub enum Error {
    NotFound(String),
    Network(String),
    System(String),
}

impl Error {
    pub fn new<T: Into<String>>(from: T) -> Self {
        Error::System(from.into())
    }
}

pub type CommandFuture<I, E = Error> = Box<Future<Item = I, Error = E>>;
pub type Result<T, E = Error> = std::result::Result<T, E>;

impl From<MailboxError> for Error {
    fn from(_error: MailboxError) -> Self {
        Error::new("Actix mailbox error")
    }
}

impl From<actix_web::Error> for Error {
    fn from(error: actix_web::Error) -> Self {
        Error::Network(format!("{:?}", error))
    }
}
