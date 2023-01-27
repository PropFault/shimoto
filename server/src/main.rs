mod endpoints;
mod dao;

use actix_web::{HttpServer, App, middleware};
use std::{env, io};
#[actix_web::main]
async fn main() -> io::Result<()>{
    env::set_var("RUST_LOG", "actix_web=debug,actix_server=info");
    env_logger::init();

    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Logger::default())
            .service(endpoints::comic::read)
    })
        .bind("0.0.0.0:9090")?
        .run()
        .await
}
