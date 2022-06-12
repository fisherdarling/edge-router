use core::fmt;
use std::pin::Pin;

use bytes::Buf;
pub use http_body::Body as HttpBody;

pub type Body = LocalBody<bytes::Bytes, crate::Error>;

pub struct LocalBody<D, E> {
    inner: Pin<Box<dyn HttpBody<Data = D, Error = E> + 'static>>,
}

impl<D, E> fmt::Debug for LocalBody<D, E> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("LocalBody").finish()
    }
}

impl<D, E> HttpBody for LocalBody<D, E>
where
    D: Buf,
{
    type Data = D;

    type Error = E;

    fn poll_data(
        mut self: Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Option<Result<Self::Data, Self::Error>>> {
        self.inner.as_mut().poll_data(cx)
    }

    fn poll_trailers(
        mut self: Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Result<Option<http::HeaderMap>, Self::Error>> {
        self.inner.as_mut().poll_trailers(cx)
    }
}
