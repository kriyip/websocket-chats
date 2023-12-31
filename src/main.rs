use actix::*;
use actix_cors::Cors;
use actix_files::Files;
use actix_web::{web, http, App, HttpServer};
use diesel::{
    prelude::*,
    r2d2::{self, ConnectionManager},
};
mod routes;

#[actix_web::main]
async fn main() -> std::io::Result<()>{
    println!("Hello, world!");

    unimplemented!()
}
