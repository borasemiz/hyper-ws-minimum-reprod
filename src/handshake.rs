use hyper::{Body, Request, Response};

pub fn do_handshake(request: &Request<Body>, response: &mut Response<Body>) -> Result<(), ()> {
  Ok(())
}