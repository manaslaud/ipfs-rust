use actix_web::HttpResponse;

#[actix_web::get("/health")]
async fn greet() -> HttpResponse {
HttpResponse::Ok().body("IPFS Node HTTP gateway running.")
}