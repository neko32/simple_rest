use actix_web::*;
use once_cell::sync::Lazy;
use std::env;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {

    static CONF:Lazy<Conf> = Lazy::new(|| {
        let host = env::var("host").unwrap_or("0.0.0.0".to_string());
        let port:u16 = env::var("port").unwrap_or("8999".to_string()).parse().unwrap();
        Conf { hostname:host, port:port }
    });

    let http = HttpServer::new(|| {
        App::new()
        .service(hello_handler)
    }).bind((CONF.hostname.as_str(), CONF.port))?;
    http.run().await
}

#[get("/hello/{name}")]
async fn hello_handler(name:web::Path<String>) -> impl Responder {
    let msg = format!("<p>hello <strong>{}</strong></p>", name.as_str()); 
    HttpResponse::Ok()
    .content_type("text/html")
    .body(msg)
}

#[derive(Debug)]
struct Conf {
    hostname:String,
    port:u16,
}