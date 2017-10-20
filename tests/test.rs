#![feature(plugin)]
#![plugin(include_dir_bytes)]

extern crate iron;
extern crate iron_test;
extern crate embed_staticfile;

use iron::headers::{Headers};
use iron_test::{request, ProjectBuilder};

use std::str;

#[test]
fn embed_staticfile_test() {
    let p = ProjectBuilder::new("example");
    p.build();
    let st = embed_staticfile::EmbedStatic::new(include_dir!("../assets"));
    match request::get("http://localhost:3000/a.txt", Headers::new(), &st) {
        Ok(res) => {
            let mut body = Vec::new();
            res.body.unwrap().write_body(&mut body).unwrap();
            assert_eq!(str::from_utf8(&body).unwrap(), "test\n");
        },
        Err(e) => panic!("{}", e)
    }
}
