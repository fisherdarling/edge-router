use crate::body::Body;

pub type Response<B = Body> = http::Response<B>;
