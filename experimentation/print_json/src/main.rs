use dotenvy::dotenv;

use actix_web::{HttpResponse};
//use serde::Deserialize;
//
use actix_web::{
    get, post,
    web::{Data}
};
//
//use diesel::pg::PgConnection;
//use diesel::r2d2;
//
//
//type Pool = r2d2::Pool<r2d2::ConnectionManager<PgConnection>>;
//
//pub fn init_pool(database_url: &str) -> Pool {
//    let manager = r2d2::ConnectionManager::<PgConnection>::new(database_url);
//    let pool = Pool::new(manager).expect("DB pool failed");
//    pool
//}
//
//#[actix_web::main]
//async fn main() -> std::io::Result<()> {
//    dotenv().ok();
//    let database_url = std::env::var("DATABASE_URL").expect("Didn't load the URL properly");
//    let pool = init_pool(&database_url);
//    HttpServer::new(move || App::new().service(index).service(insert).app_data(Data::new(pool.clone())))
//        .bind(("127.0.0.1", 8080))?
//        .run()
//        .await
//}
//
//#[get("/")]
//async fn index() -> impl Responder {
//    dbg!("Heellloo");
//    HttpResponse::Ok().body("Hello world!")
//}
//
//#[derive(Deserialize, Debug)]
//struct FormData {
//    username: String,
//}
//
//#[post("/post")]
//async fn insert(form: web::Json<FormData>) -> HttpResponse {
//    dbg!(&form);
//    HttpResponse::Ok().body(format!("username: {}", form.username))
//}
//
//#[actix_web::post("/post1")]
//async fn insert_new_hive(
//    data: web::Json<FormData>,
//) -> impl Responder {
//    dbg!(&data);
//}

use actix_web::{web, App, HttpServer, Result};
use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct Info {
    username: String,
}

#[derive(Deserialize, Debug)]
struct Posto {
    info: Info,
    number: i32
}

/// extract `Info` using serde
async fn index(posto: web::Json<Posto>) -> Result<String> {
    dbg!(&posto.info.username);
    Ok(format!("Welcome {}!", posto.info.username))
}

#[post("/post1")]
async fn insert(posto: web::Json<Posto>) -> HttpResponse {
    dbg!(&posto);
    // Ok(format!("username: {}", info.username))
    HttpResponse::Ok().body(format!("username: {}", posto.info.username))
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(insert).route("/", web::post().to(index)))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
