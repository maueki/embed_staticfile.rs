extern crate iron;
extern crate hyper;
extern crate conduit_mime_types as mime_types;
#[macro_use]
extern crate lazy_static;

use iron::prelude::*;
use iron::status;
use iron::Handler;
use std::path::Path;
use hyper::mime::Mime;

use std::collections::HashMap;

lazy_static! {
    static ref MIME_TYPES: mime_types::Types = mime_types::Types::new().unwrap();
}

#[derive(Clone)]
pub struct EmbedStatic {
    map: HashMap<String, Vec<u8>>,
}

impl EmbedStatic {
    pub fn new(map: HashMap<String, Vec<u8>>) -> EmbedStatic {
        EmbedStatic { map: map }
    }
}

impl Handler for EmbedStatic {
    fn handle(&self, req: &mut Request) -> IronResult<Response> {
        let path: String = req.url.path().pop().unwrap().to_string();
        match self.map.get(&path) {
            Some(b) => {
                let mut res = Response::with((status::Ok, b.as_slice()));

                let mime_str = MIME_TYPES.mime_for_path(Path::new(&path));
                let _ = mime_str.parse().map(|mime: Mime| res.set_mut(mime));
                Ok(res)
            }
            None => Ok(Response::with((status::NotFound))),
        }
    }
}
