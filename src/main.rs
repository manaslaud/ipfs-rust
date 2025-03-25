use actix_web::{App, HttpServer};
use ipfs_rust::constants::constants::_PORT;
use ipfs_rust::network::http_gateway::upload_file::upload_file;
#[actix_web::main]
pub async fn main()  {
    //defining and spinning up the http server
    let server = HttpServer::new(|| App::new().service(upload_file))
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