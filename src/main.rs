use actix_cors::Cors;
use actix_web::{http::header, App, HttpServer};
use ipfs_rust::constants::constants::_PORT;
use ipfs_rust::network::http_gateway::health::greet;
use ipfs_rust::network::http_gateway::upload::upload;
#[actix_web::main]
pub async fn main() {
    //defining and spinning up the http server
    let server = HttpServer::new(|| {
        let cors = Cors::default()
            .allowed_origin("http://127.0.0.1:5500") // or "*" if you're testing
            .allowed_methods(vec!["GET", "POST"])
            .allowed_headers(vec![header::CONTENT_TYPE, header::ACCEPT])
            .supports_credentials() // optional, only if you're sending cookies
            .max_age(3600);
        App::new().wrap(cors).service(upload).service(greet)
    })
    .bind(("127.0.0.1", _PORT));
    match server {
        Ok(server) => {
            if let Err(e) = server.workers(2).run().await {
                eprintln!("Server error: {:?}", e);
            }
        }
        Err(e) => eprintln!("Failed to bind server: {:?}", e),
    };
}
