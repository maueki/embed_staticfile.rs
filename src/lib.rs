extern crate iron;
extern crate hyper;
extern crate conduit_mime_types as mime_types;
#[macro_use]
extern crate lazy_static;

use iron::prelude::*;
use iron::status;
use iron::Handler;
use iron::url::percent_encoding::percent_decode;
use hyper::mime::Mime;

use std::iter::FromIterator;
use std::path::{Path, PathBuf};
use std::collections::HashMap;

lazy_static! {
    static ref MIME_TYPES: mime_types::Types = mime_types::Types::new().unwrap();
}

fn decode_percents(string: &&str) -> String {
    percent_decode(string.as_bytes()).decode_utf8().unwrap().into_owned()
}

#[derive(Clone)]
pub struct EmbedStatic {
    map: HashMap<&'static Path, Vec<u8>>,
}

impl EmbedStatic {
    pub fn new(map: HashMap<&'static Path, Vec<u8>>) -> EmbedStatic {
        EmbedStatic { map: map }
    }
}

impl Handler for EmbedStatic {
    fn handle(&self, req: &mut Request) -> IronResult<Response> {
        let req_path = PathBuf::from_iter(req.url.path().iter().map(decode_percents));
        match self.map.get(req_path.as_path()) {
            Some(b) => {
                let mut res = Response::with((status::Ok, b.as_slice()));

                let mime_str = MIME_TYPES.mime_for_path(req_path.as_path());
                let _ = mime_str.parse().map(|mime: Mime| res.set_mut(mime));
                Ok(res)
            }
            None => Ok(Response::with((status::NotFound))),
        }
    }
}
