use actix_web::{
    web, HttpResponse,
};
use serde::Deserialize;
use chrono::NaiveDateTime;
use diesel::PgConnection;
use diesel::r2d2::ConnectionManager;
use crate::server::errors::Result;
use super::db::{
    LoggingLevel,
    LogNote,
    NewLogNote
};
use std::str::FromStr;
pub type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

pub fn logs_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/logs")
        .route("", web::get().to(get_logs))
        .route("/range", web::get().to(get_logs_range))
        .route("", web::post().to(create_log))
    );
}

pub async fn get_logs(
        conn: web::Data<DbPool>,
    ) -> Result<HttpResponse> {
        let conn = conn.get()?;
        let data = LogNote::get(&conn).await?;
        Ok(HttpResponse::Ok().json(data))
    }

#[derive(Deserialize)]
pub struct GetLogsRange {
    pub from_time: String,
    pub to_time: String,
    pub logging_level: String,
}

pub async fn get_logs_range(
    form: web::Query<GetLogsRange>,
    conn: web::Data<DbPool>,
) -> Result<HttpResponse> {
    let conn = conn.get()?;
    let data = LogNote::get_range(
            NaiveDateTime::parse_from_str(&form.from_time, "%Y-%m-%dT%H:%M:%S%.f")?,
            NaiveDateTime::parse_from_str(&form.to_time, "%Y-%m-%dT%H:%M:%S%.f")?,
            LoggingLevel::from_str(&form.logging_level)?,
            &conn
        )
        .await?;
    Ok(HttpResponse::Ok().json(data))
}


pub async fn create_log(
    form: web::Json<NewLogNote>,
    conn: web::Data<DbPool>,
) -> Result<HttpResponse> {
    let conn = conn.get()?;
    let form = form.into_inner();
    LogNote::new(&form, &conn).await?;
    Ok(HttpResponse::Ok().json(""))
}