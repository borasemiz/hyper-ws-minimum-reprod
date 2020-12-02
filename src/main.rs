mod handshake;
mod handler;

use hyper::Server;

#[tokio::main]
async fn main() {
    let addr = ([127, 0, 0, 1], 9014).into();
    let handler_inst = handler::make_handler();
    let server = Server::bind(&addr).serve(handler_inst);
    server.await.unwrap();
}
