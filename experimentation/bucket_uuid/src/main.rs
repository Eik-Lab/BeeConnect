use dotenvy::dotenv;
use models::Measurement;

use actix_web::{
    get, post,
    web::{self, Data},
    App, HttpResponse, HttpServer, Responder,
};

use awscreds::Credentials;
use awsregion::Region;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::r2d2;
use s3::error::S3Error;
use s3::Bucket;

pub mod models;
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
    HttpServer::new(move || App::new().service(index).app_data(Data::new(pool.clone())))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}

#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello world")
}

//#[post("/post")]
//async fn insert(web::Json(info): web::Json<Measurement>) -> String {
//  dbg!(&info);
////    format!("Welcome {}!", info.post.id)
//}

#[actix_web::post("/post")]
async fn insert_measurement(
    web::Json(data): web::Json<Measurement>,
    pool: web::Data<Pool>,
) -> impl Responder {
    let conn = &mut pool.get().unwrap();
    dbg!(&data);
    diesel::insert_into(crate::schema::measurements::table)
        .values(data.post)
        .execute(conn)
        .expect("Could not post data");
    HttpResponse::Ok().finish()
}
