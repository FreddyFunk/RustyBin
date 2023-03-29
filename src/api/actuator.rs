use actix_web::{
    get,
    HttpResponse,
    Responder,
};

#[get("/version")]
pub async fn version() -> impl Responder {
    const VERSION: &str = env!("CARGO_PKG_VERSION");
    return HttpResponse::Ok().body(VERSION);
}

#[get("/health_check")]
pub async fn health_check() -> impl Responder {
    const HEALTH: &str = "OK";
    return HttpResponse::Ok().body(HEALTH);
}