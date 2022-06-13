use std::convert::Infallible;

use kante_core::{Body, LocalService, Request, Response};
use tower_service::Service;

pub type Route<B = Body, E = Infallible> = LocalService<Request<B>, Response, E>;

// #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
// pub struct RouteId(u16);

// pub struct RouteList(Vec<Route>);

pub struct Router {
    // routes: RouteList,
    inner: matchit::Router<Route>,
}

impl Router {
    pub fn new() -> Self {
        Self {
            inner: matchit::Router::new(),
        }
    }

    pub fn route<T>(mut self, path: &str, service: T) -> Self
    where
        T: Service<Request<Body>, Response = Response, Error = Infallible> + 'static,
        T::Future: 'static,
    {
        let service = Route::new(service);

        self.inner
            .insert(path, service)
            .expect("unable to insert route");

        self
    }
}
