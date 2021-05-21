use actix_web::HttpResponse;

pub async fn get_version() -> HttpResponse {
    HttpResponse::Ok().json(json!({"version": "1.0.0"}))
}

pub async fn get_health() -> HttpResponse {
    HttpResponse::Ok().json(json!({}))
}