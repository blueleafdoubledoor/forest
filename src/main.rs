extern crate iron;
extern crate mount;
extern crate router;
extern crate staticfile;

use iron::status;
use iron::{Iron, Request, Response, IronResult};

use mount::Mount;
use router::Router;
use staticfile::Static;

use std::path::Path;

fn say_hello(req: &mut Request) -> IronResult<Response> {
    println!("Running send_hello handler, URL path: {}",
             req.url.path().join("/"));
    Ok(Response::with((status::Ok, "This request was routed!")))
}

fn main() {
    let mut router = Router::new();
    router.get("/hello", say_hello, "hello");

    let mut mount = Mount::new();
    mount.mount("/", Static::new(Path::new("static/index.html")));
    let host = "localhost:3000";
    Iron::new(mount).http(host).unwrap();
}
