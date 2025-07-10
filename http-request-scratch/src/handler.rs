use http_body_util::Full;
use hyper::body::Bytes;
use hyper::{Request, Response};
use std::convert::Infallible;


// This is our method to handle HTTP connections
pub async fn handle(body: Request<hyper::body::Incoming>)
    -> Result<Response<Full<Bytes>>, Infallible> {
    println!("Received a request");
    println!("Headers: {:?}", body.headers());
    println!("Body: {:?}", body.into_body());
    Ok(Response::new(Full::new(Bytes::from("Hello, World!"))))
}