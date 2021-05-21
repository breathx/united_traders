use actix_web::{
    web, HttpResponse, HttpRequest,
};

pub fn server_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/server")
        .route("/version", web::get().to(get_version))
        .route("/health", web::get().to(get_health))
    );
}

pub async fn get_version(
    _req: HttpRequest
) -> HttpResponse {
    HttpResponse::Ok().json("")
}

pub async fn get_health(
    _req: HttpRequest
) -> HttpResponse {
    HttpResponse::Ok().json("")
}