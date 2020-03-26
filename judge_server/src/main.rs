use std::convert::Infallible;
use tokio::prelude::*;
use pretty_env_logger;
use std::net::SocketAddr;
use hyper::{Body, Request, Response, Server};
use hyper::service::{make_service_fn, service_fn};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error+Send+Sync>> {
    pretty_env_logger::init();

    let addr = ([127,0,0,1], 443).into();

    let make_svc = make_service_fn(|_conn| {
       async { Ok::<_, Infallible>(service_fn(hello)) }
    });

    let server = Server::bind(&addr).serve(make_svc);

    println!("Listening on https://{}", addr);

    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    }
    Ok(())
    
}

async fn hello(_: Request<Body>) -> Result<Response<Body>, Infallible> {
    Ok(Response::new(Body::from("Hello World!")))
}
