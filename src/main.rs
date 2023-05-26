// #![allow(unused)] // For beginning only.

mod handler;
mod model;
mod route;
mod schema;

use std::{net::SocketAddr, sync::Arc};

use axum::http::{
    header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE},
    HeaderValue, Method,
};
use dotenv::dotenv;
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};
use tokio::sync::RwLock;
use tower_http::cors::CorsLayer;
// use tower_http::cors::{Any, CorsLayer};

use crate::route::create_router;

pub struct AppState {
    db: Pool<Postgres>,
}

#[tokio::main]
async fn main() {
    dotenv().ok();

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let pool = match PgPoolOptions::new()
        .max_connections(10) // https://docs.rs/sqlx/latest/sqlx/pool/struct.PoolOptions.html#method.max_connections
        .connect(&database_url)
        .await
    {
        Ok(pool) => {
            println!("->> ✅Connection to the database is successful!\n");
            pool
        }
        Err(err) => {
            println!("->> 🔥 Failed to connect to the database: {:?}\n", err);
            std::process::exit(1);
        }
    };

    let app_state = Arc::new(RwLock::new(AppState { db: pool.clone() }));

    // Set up of Cross-Origin Resource Sharing (CORS) to allow the server to
    // accept cross-origin requests from specified origins ("http://localhost:3000").
    let cors = CorsLayer::new()
        .allow_origin("http://localhost:3000".parse::<HeaderValue>().unwrap())
        .allow_methods([Method::POST, Method::GET, Method::PATCH, Method::DELETE])
        .allow_credentials(true)
        .allow_headers([AUTHORIZATION, ACCEPT, CONTENT_TYPE]);

    // An alternative configuration that allows any origin, any method, and any type of header would be:
    // let cors = CorsLayer::new()
    //     .allow_origin(Any)
    //     .allow_methods(Any)
    //     .allow_headers(Any);

    let app = create_router(app_state).layer(cors);

    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
    println!("->> LISTENING on {addr}\n");

    println!("🚀 Server started successfully!!\n");

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

/*
 * COMANDO PARA VER LA ADDRESS-IP (variable) DEL CONTENEDOR postgres:
 * sudo docker inspect postgres
 *
 * https://codevoweb.com/rust-crud-api-example-with-axum-and-postgresql/
 * https://github.com/wpcodevo/rust-axum-postgres-api
 * https://github.com/launchbadge/sqlx/tree/main/sqlx-cli
 *
 * PASANDO VARIOS PARÁMETROS QUERY AL COMANDO cURL. VER:
 * https://stackoverflow.com/questions/2981923/how-to-pass-multiple-parameters-to-cron-job-with-curl#53023315
 *
 * SIRVIENDO ESTÁTICOS DESDE AXUM. VER:
 * https://github.com/search?q=axum+blog&type=repositories
 * https://github.com/Ericarthurc/ericarthurc.com_axum_solid_OLD
 * https://github.com/Ericarthurc/ericarthurc.com_axum_OLD
 *
 * https://github.com/search?q=axum+solidjs&type=repositories
 * https://github.com/robertwayne/template-axum-solidjs-spa
 *
 * https://github.com/search?q=axum%20react&type=repositories
 * https://github.com/robertwayne/template-axum-react-spa
 *
 * https://github.com/search?q=axum%20yew&type=repositories
 * https://github.com/rksm/axum-yew-setup
 * https://robert.kra.hn/posts/2022-04-03_rust-web-wasm/
 * https://www.udemy.com/course/learn-full-stack-rust-programming-using-axum-yew-and-sqlx/
 * https://github.com/infinityfish/fullstackrustcourse
 *
 * https://www.google.com/search?q=axum+server+frontend&oq=axu&aqs=chrome.0.69i59l2j69i57j69i59j46i67i340i650j69i60l3.1955j0j4&sourceid=chrome&ie=UTF-8
 *
 * HTTP Cache Headers - A Complete Guide. VER:
 * https://www.keycdn.com/blog/http-cache-headers#:~:text=downloaded%20every%20time.-,max%2Dage,for%20the%20next%2090%20seconds.
 *
 * STRINGS: HEAP OR STACK. VER:
 * https://www.google.com/search?q=rust+immutable+string+heap+or+stack&oq=rust+immu&aqs=chrome.1.69i57j35i39j0i19i512l8.47080j0j9&sourceid=chrome&ie=UTF-8
 * https://stackoverflow.com/questions/24158114/what-are-the-differences-between-rusts-string-and-str#:~:text=Rust's%20owned%20String%20type%2C%20the,the%20heap%20will%20be%20freed.
 * https://dev.to/somedood/optimizing-immutable-strings-in-rust-2ahj
 */

/*
 * COMMANDS WITH cURL TO PERFORM A CRUD TO THE API (EXAMPLES):
 *
 * curl -v -X POST http://localhost:8080/api/notes -d '{"title": "Creando nota con cURL", "content": "Funcionará ahora?", "category": "FastAPI"}' -H "content-type: application/json" | json_pp
 *
 * curl -v http://localhost:8080/api/notes | json_pp
 *
 * curl -v http://localhost:8080/api/notes?page=2\&limit=1 | json_pp
 *
 * curl -v http://localhost:8080/api/notes/043d31a6-6b27-4814-9c64-00988f8e28af | json_pp
 *
 * curl -v -X PATCH http://localhost:8080/api/notes/043d31a6-6b27-4814-9c64-00988f8e28af -d '{"title": "Editando nota con cURL", "content": "Todo está funcionando hasta el momento 😀😀", "category": "Custom FastAPI", "published" : true}' -H "content-type: application/json" | json_pp
 *
 * curl -v -X DELETE http://localhost:8080/api/notes/1dc89584-6868-4c17-a21e-3f592617f917
 */
