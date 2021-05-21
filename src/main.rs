use actix_web::{
    web, HttpServer, App, 
};
use actix_web::middleware::Logger;
use diesel::PgConnection;
use diesel::r2d2::ConnectionManager;
use diesel_migrations::run_pending_migrations;
use united_traders::server::routes::{
    get_version,
    get_health,
};


#[actix_web::main]
async fn main() -> std::io::Result<()> {

    let database_url = std::env::var("DATABASE_URL").expect("ERROR: variable DATABASE_URL is not set");
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("ERROR: Failed to create pool");

    match run_pending_migrations(&pool.get().unwrap()) {
        Ok(_) => println!("Migrations were successful"),
        Err(e)=> println!("Migrations failed with an error: {}", &e),
    };

    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    println!("The server is starting up!");
    HttpServer::new(move || {
        App::new()
            .data(pool.clone())
            .wrap(Logger::default())
            .service(web::scope("/api")
                .route("/version", web::get().to(get_version))
                .route("/health", web::get().to(get_health))
                // .configure(server_routes)
            )
    })
    .bind("0.0.0.0:8088")?
    .system_exit()
    .run()
    .await
}
