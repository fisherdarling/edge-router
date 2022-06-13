pub(crate) mod body;
pub(crate) mod error;
pub(crate) mod macros;
pub(crate) mod map_future;
pub(crate) mod response;
pub(crate) mod service;

pub use crate::{
    body::Body,
    error::{BoxError, Error},
    response::Response,
    service::LocalService,
};

pub type Request<B> = http::Request<B>;
