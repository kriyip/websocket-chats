// contains all the routes for the server
use std::time::Instant;
use actix::*;
use actix_files::NamedFile;
use actix_web::{get, post, web, Error, HttpRequest, HttpResponse, Responder};
use actix_web_actors::ws;
use diesel::{
    prelude::*,
    r2d2::{self, ConnectionManager}, sqlite::Sqlite,
};
use serde_json::json;

// implementing Responder, an actix-web trait that converts a type into an HTTP response
// serves static index.html file
pub async fn index() -> impl Responder {
    NamedFile::open("static/index.html")
}

// params:
// req_details: HttpRequest - contains information about the request
// stream: web::Payload - a stream of bytes representing the request body
// pool: web::Data<Pool> - a connection pool to the database
// returns: impl Responder - a trait that converts a type into an HTTP response

pub async fn connect(
    req_details: HttpRequest,
    stream: web::Payload,
    pool: web::Data<r2d2::Pool<ConnectionManager<SqliteConnection>>>,
    // srv: web::Data<Addr<server::ChatServer>>,
) {
    

}