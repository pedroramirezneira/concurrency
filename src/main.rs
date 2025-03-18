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
    server.get("/pi/:a", |context| {
        let number = context.get_request().get_param("a").unwrap();
        let number = number.parse::<f64>();
        match number {
            Err(_) => {
                context.set_status(HttpStatusCode::BadRequest);
                context.send_text("Invalid number");
            }
            Ok(number) => {
                context.set_status(HttpStatusCode::Ok);
                context.send_text(&(number * 3.14159).to_string());
            }
        }
    });
    let server = server;
    server.serve(5000);
}
