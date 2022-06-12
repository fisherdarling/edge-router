use std::convert::Infallible;

use tower_service::Service;

use crate::{body::Body, service::LocalService, Request, Response};

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
    //
    // pub fn route()
}
