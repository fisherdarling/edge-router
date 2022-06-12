use futures_util::future::BoxFuture;
use tower_service::Service;

pub struct LocalService<T, U, E>(
    Box<dyn Service<T, Response = U, Error = E, Future = BoxFuture<'static, Result<U, E>>>>,
);

impl<T, U, E> Service<T> for LocalService<T, U, E> {
    type Response = U;

    type Error = E;

    type Future = BoxFuture<'static, Result<U, E>>;

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
