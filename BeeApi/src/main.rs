use actix_web::{
    get, post,
    web::{self, Data},
    App, HttpResponse, HttpServer, Responder,
};

use diesel::r2d2::ConnectionManager;
//use r2d2_diesel::ConnectionManager;

use diesel::r2d2;
use models::Hello;
use serde_json::{from_value, json};

use anyhow::Result;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenvy::dotenv;
use std::env;

pub mod models;
pub mod schema;
type Pool = r2d2::Pool<r2d2::ConnectionManager<PgConnection>>;

/// Initialize data pool and return it
pub fn init_pool(database_url: &str) -> Pool {
    let manager = r2d2::ConnectionManager::<PgConnection>::new(database_url);
    let pool = Pool::new(manager).expect("DB pool failed");
    pool
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let database_url = std::env::var("DATABASE_URL").expect("Didn't load the URL properly");
    let pool = init_pool(&database_url);
    HttpServer::new(move || {
        App::new()
            .service(hello)
            .service(insert_hello)
            .service(get_data)
            .app_data(Data::new(pool.clone()))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[actix_web::post("/data")]
async fn insert_hello(web::Json(data): web::Json<Hello>, pool: web::Data<Pool>) -> impl Responder {
    let conn = &mut pool.get().unwrap();
    dbg!(&data);
    diesel::insert_into(crate::schema::hellos::table)
        .values(data)
        .execute(conn)
        .expect("Could not post data");
    HttpResponse::Ok().finish()
}

#[actix_web::get("/data")]
async fn get_data(pool: web::Data<Pool>) -> impl Responder {
    let conn = &mut pool.get().unwrap();
    //let data = hellos.load::<Hello>(conn).expect("Error loading hellos");
    let data = crate::schema::hellos::table
        .load::<Hello>(conn)
        .expect("Error loading hellos");
    dbg!(&data);
    actix_web::HttpResponse::Ok().json(data)
}
