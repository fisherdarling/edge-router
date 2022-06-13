// use futures_util::future::BoxFuture;
use std::{future::Future, pin::Pin};
use tower_service::Service;

pub type BoxFuture<T, E> = Pin<Box<dyn Future<Output = Result<T, E>> + 'static>>;

pub struct LocalService<T, U, E>(
    Box<dyn Service<T, Response = U, Error = E, Future = BoxFuture<U, E>>>,
);

impl<T, U, E> LocalService<T, U, E> {
    pub fn new<S>(inner: S) -> Self
    where
        S: Service<T, Response = U, Error = E> + 'static,
        S::Future: 'static,
    {
        LocalService(Box::new(crate::map_future::MapFuture::new(inner, |fut| {
            Box::pin(fut) as _
        })))
    }
}

impl<T, U, E> Service<T> for LocalService<T, U, E> {
    type Response = U;

    type Error = E;

    type Future = BoxFuture<U, E>;

    fn poll_ready(
        &mut self,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Result<(), Self::Error>> {
        self.0.poll_ready(cx)
    }

    fn call(&mut self, req: T) -> Self::Future {
        self.0.call(req)
    }
}
