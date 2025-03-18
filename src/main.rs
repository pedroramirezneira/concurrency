mod http;
mod response;
mod server;

use http::http_status_code::HttpStatusCode;
use server::web_server::WebServer;

fn main() {
    let mut server = WebServer::new();
    server.get("/", |context| {
        context.set_status(HttpStatusCode::Ok);
        context.send_text("Hello World");
    });
    let server = server;
    server.serve(5000);
}
