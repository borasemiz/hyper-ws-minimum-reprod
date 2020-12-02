use hyper::upgrade::Upgraded;
use hyper::service::Service;
use hyper::{Body, Request, Response};

use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};
use std::sync::{Arc};

use crate::handshake;

pub fn make_handler() -> MakeHandler {
  MakeHandler{}
}

pub struct Handler {
  streams: Arc<Option<Upgraded>>
}

impl Handler {
  pub fn new() -> Self {
    Handler { streams: Arc::new(None) }
  }
}

impl Service<Request<Body>> for Handler {
  type Response = Response<Body>;
  type Error = hyper::Error;
  type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send>>;

  fn poll_ready(&mut self, _: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
    Poll::Ready(Ok(()))
  }

  fn call(&mut self, req: Request<Body>) -> Self::Future {
    let mut res = Response::new(Body::empty());

    if let Ok(_) = handshake::do_handshake(&req, &mut res) {
      tokio::task::spawn(async move {
        let upgraded = match req.into_body().on_upgrade().await {
          Ok(upgraded) => upgraded,
          Err(e) => {
            eprintln!("upgrade error: {}", e);
            return Err(Box::new(e));
          },
        };

        Ok(())
      });
    }

    Box::pin(async { Ok(res) })
  }
}

pub struct MakeHandler;

impl<T> Service<T> for MakeHandler {
  type Response = Handler;
  type Error = hyper::Error;
  type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send>>;

  fn poll_ready(&mut self, _: &mut Context) -> Poll<Result<(), Self::Error>> {
    Poll::Ready(Ok(()))
  }

  fn call(&mut self, _: T) -> Self::Future {
    let fut = async move { Ok(Handler::new()) };
    Box::pin(fut)
  }
}
