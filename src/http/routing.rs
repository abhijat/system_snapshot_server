use std::collections::HashMap;

use iron::prelude::*;
use iron::Handler;
use iron::status;

pub struct Router {
    routes: HashMap<String, Box<Handler>>
}

impl Router {
    pub fn new() -> Self {
        Router { routes: HashMap::new() }
    }

    pub fn add_route<H>(&mut self, path: String, handler: H) where H: Handler {
        self.routes.insert(path, Box::new(handler));
    }
}

impl Handler for Router {
    fn handle(&self, req: &mut Request) -> IronResult<Response> {

        let url = &req.url.path().join("/");
        debug!("matching {} to route map", url);

        match self.routes.get(url) {
            Some(handler) => handler.handle(req),
            None => {
                error!("unknown url {}", url);
                Ok(Response::with((status::NotFound, "url not found!")))
            }
        }
    }
}
