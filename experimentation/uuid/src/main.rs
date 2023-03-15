use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::r2d2;
use dotenvy::dotenv;

pub mod models;
pub mod schema;
use crate::{models::*, schema::*};

use actix_web::{
    get, post,
    web::{self, Data},
    App, HttpResponse, HttpServer, Responder,
};

type Pool = r2d2::Pool<r2d2::ConnectionManager<PgConnection>>;

pub fn establish_connection(database_url: &str) -> Pool {
    let manager = r2d2::ConnectionManager::<PgConnection>::new(database_url);
    let pool = Pool::new(manager).expect("DB pool faild");
    pool
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let database_url = std::env::var("DATABASE_URL").expect("Didn't load the URL properly");
    let pool = establish_connection(&database_url);
    HttpServer::new(move || {
        App::new()
            .service(index)
            .service(insert_new)
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

#[post("/post")]
async fn insert_new(
    web::Json(data): web::Json<models::NewMes>,
    pool: web::Data<Pool>,
) -> impl Responder {
    let conn = &mut pool.get().unwrap();
    dbg!(&data);

    //let data2 = models::PostUuid::from(&data);
    diesel::insert_into(crate::schema::post_uuids::table)
        .values(models::PostUuid::from(data))
        .execute(conn)
        .expect("Could not post data");
    HttpResponse::Ok().body("This is what we want")
}

#[get("/get")]
async fn get_dat(pool: web::Data<Pool>) -> actix_web::HttpResponse {
    let conn = pool.get().unwrap();
    actix_web::HttpResponse::Ok().finish()
}

//#[get("/get")]
//async fn get_data(pool: web::Data<Pool>) -> impl Responder {
//    use self::schema::post_uuids::dsl::*;
//    let conn = &mut pool.get().unwrap();
//    let items = post_uuids.load::<PostUuid>(conn).unwrap();
//    HttpResponse::Ok().finish()
//}
