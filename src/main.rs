mod http;
mod response;
mod server;

use std::time::Instant;
use http::http_status_code::HttpStatusCode;
use server::web_server::WebServer;
use response::pi::leibniz;

fn main() {
    let mut server = WebServer::new();
    server.get("/", |context| {
        context.set_status(HttpStatusCode::Ok);
        context.send_text("Hello World");
    });
    server.get("/pi/:a", |context| {
        let start = Instant::now();
        let number = context.get_request().get_param("a").unwrap();
        let number = number.parse::<u64>();
        match number {
            Err(_) => {
                context.set_status(HttpStatusCode::BadRequest);
                context.send_text("Invalid number");
            }
            Ok(number) => {
                context.set_status(HttpStatusCode::Ok);
                let pi = leibniz(number);
                let elapsed = start.elapsed().as_secs_f64();
                let result = format!("Valor de pi para el termino {}: {} (Tiempo: {})",
                                     number, pi.to_string(), &elapsed.to_string());
                context.send_text(&*result);
            }
        }
    });
    let server = server;
    server.serve(5000);
}
