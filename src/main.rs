#![allow(dead_code)]
#[allow(unused_imports)]
mod functions;
mod models;
mod http_render;
mod schema;
#[macro_use]
extern crate diesel;
use actix_web::{App, HttpServer, Responder, HttpResponse, get};
use dotenv::dotenv;
use std::{thread, time};
use std::time::Duration;
use log::{error, info, warn, debug, trace};
use log4rs;


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    log4rs::init_file("config/log4rs.yaml", Default::default()).unwrap();
    HttpServer::new(move || {
        App::new()
            .service(http_render::insert_f_file)
            .service(http_render::insert_f_file_post)
            .service(http_render::login_check)
            .service(http_render::login_check_post)
            .service(http_render::signup)
            .service(http_render::signup_post)
            .service(http_render::signup_admin)
            .service(http_render::signup_admin_post)
            .service(http_render::gwanho)
    })
    .bind("0.0.0.0:8181")?
    .run()
    .await
}
