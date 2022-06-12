pub(crate) mod body;
pub(crate) mod error;
pub(crate) mod response;
pub(crate) mod router;
pub(crate) mod service;

use crate::body::Body;

pub use crate::error::{BoxError, Error};

pub type Request<B> = http::Request<B>;
pub type Response<B = Body> = http::Response<B>;
// pub type Response<B =
// pub type Request<B> = http::Request<B>;
