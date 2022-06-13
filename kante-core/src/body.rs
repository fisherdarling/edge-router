use core::fmt;
use std::pin::Pin;

use bytes::{Buf, BufMut, Bytes};
pub use http_body::Body as HttpBody;

use crate::BoxError;

pub type Body = LocalBody<Bytes, crate::Error>;

pub struct LocalBody<D, E> {
    inner: Pin<Box<dyn HttpBody<Data = D, Error = E> + 'static>>,
}

impl<D, E> LocalBody<D, E> {
    pub fn new(inner: impl HttpBody<Data = D, Error = E> + 'static) -> Self {
        Self {
            inner: Box::pin(inner),
        }
    }
}

impl<D, E> fmt::Debug for LocalBody<D, E> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("LocalBody").finish()
    }
}

pub fn boxed<B>(body: B) -> Body
where
    B: HttpBody<Data = Bytes> + 'static,
    B::Error: Into<BoxError>,
{
    try_downcast(body).unwrap_or_else(|body| LocalBody::new(body.map_err(crate::Error::new)))
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

// `try_downcast` is adapted from the crate `axum`
// under the following license:
/*
Copyright 2021 Axum Contributors

Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation files (the "Software"), to deal in the Software without restriction, including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is furnished to do so, subject to the following conditions:
The above copyright notice and this permission notice shall be included in all copies or substantial portions of the Software.
THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
*/
pub fn try_downcast<T, K>(k: K) -> Result<T, K>
where
    T: 'static,
    K: 'static,
{
    let mut k = Some(k);
    if let Some(k) = <dyn std::any::Any>::downcast_mut::<Option<T>>(&mut k) {
        Ok(k.take().unwrap())
    } else {
        Err(k.unwrap())
    }
}

// copied from hyper under the following license:
// Copyright (c) 2014-2021 Sean McArthur

// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:

// The above copyright notice and this permission notice shall be included in
// all copies or substantial portions of the Software.

// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN
// THE SOFTWARE.
pub(crate) async fn to_bytes<T>(body: T) -> Result<Bytes, T::Error>
where
    T: HttpBody,
{
    futures_util::pin_mut!(body);

    // If there's only 1 chunk, we can just return Buf::to_bytes()
    let mut first = if let Some(buf) = body.data().await {
        buf?
    } else {
        return Ok(Bytes::new());
    };

    let second = if let Some(buf) = body.data().await {
        buf?
    } else {
        return Ok(first.copy_to_bytes(first.remaining()));
    };

    // With more than 1 buf, we gotta flatten into a Vec first.
    let cap = first.remaining() + second.remaining() + body.size_hint().lower() as usize;
    let mut vec = Vec::with_capacity(cap);
    vec.put(first);
    vec.put(second);

    while let Some(buf) = body.data().await {
        vec.put(buf?);
    }

    Ok(vec.into())
}
