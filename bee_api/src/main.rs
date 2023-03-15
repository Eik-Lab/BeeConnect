use dotenvy::dotenv;
use models::Measurement;
pub mod models;

use actix_web::{
    get, post,
    web::{self, Data},
    App, HttpResponse, HttpServer, Responder,
};

use awscreds::Credentials;
use awsregion::Region;
use diesel::prelude::*;
use diesel::r2d2;
use diesel::{associations::HasTable, pg::PgConnection};
use s3::error::S3Error;
use s3::Bucket;

use crate::models::{HiveInfo, NewMeasurement};

pub mod schema;
type Pool = r2d2::Pool<r2d2::ConnectionManager<PgConnection>>;

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
            .service(index)
            .service(insert_new_hive)
            .service(insert_measurement)
            .app_data(Data::new(pool.clone()))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[actix_web::post("/post_new_hive")]
async fn insert_new_hive(
    web::Json(data): web::Json<HiveInfo>,
    pool: web::Data<Pool>,
) -> impl Responder {
    let conn = &mut pool.get().unwrap();
    diesel::insert_into(crate::schema::hive_infos::table)
        .values(data)
        .execute(conn)
        .expect("Could not post data");
    HttpResponse::Ok().finish()
}

#[actix_web::post("/post")]
async fn insert_measurement(
    web::Json(data): web::Json<NewMeasurement>,
    pool: web::Data<Pool>,
) -> impl Responder {
    let conn = &mut pool.get().unwrap();
    dbg!(&data);
    diesel::insert_into(crate::schema::measurements::table)
        .values(Measurement::from(data))
        .execute(conn)
        .expect("Could not post data");
    HttpResponse::Ok().finish()
}
