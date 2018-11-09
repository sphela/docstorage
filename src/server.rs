use configure;

use std::fs::File;
use std::io::prelude::*;

use std::cell::RefCell;
use std::rc::Rc;

use hyper::{Body, Response, Server, StatusCode};
use hyper::header::{HeaderValue};
use hyper::service::service_fn_ok;
use hyper::rt::Future;
use tokio::runtime::current_thread;

pub fn run (config: &configure::Config) {
    let doc_root = config.server.doc_root.clone();
    let addr = config.server.addr.clone();

    let index_path = format!("{}/{}", doc_root, "index.html");
    let mut f = File::open(&index_path).expect("index.html file not found");
    let mut html_contents = String::new();
    f.read_to_string(&mut html_contents)
        .expect(&format!("something went wrong reading the {} file", index_path));
    let html_contents = Rc::new(RefCell::new(html_contents));
    let new_service = move || {
        let html_c = html_contents.clone();
            service_fn_ok( move |_| {
                let c = html_c.borrow_mut();
                let mut response: Response<Body> = Response::default();
                *response.status_mut() = StatusCode::OK;
                response.headers_mut().insert("X-Thank-You", HeaderValue::from_static("For using Sphela!"));
                *response.body_mut() = Body::from(format!("{}", c));
                response
        })
    };

    let exec = current_thread::TaskExecutor::current();

    let server = Server::bind(&addr)
        .executor(exec)
        .serve(new_service)
        .map_err(|e| eprintln!("server error: {}", e));

    println!("Listening on http://{}", addr);

    current_thread::Runtime::new()
        .expect("rt new")
        .spawn(server)
        .run()
        .expect("rt run");
}
