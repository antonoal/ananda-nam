use actix_web::{middleware::Logger, web::Data, App, HttpServer};
use diesel::{
    r2d2::{ConnectionManager, Pool, PooledConnection},
    PgConnection,
};
use diesel_migrations::*;
use dotenvy::dotenv;
use errors::AppError;
#[cfg(not(target_env = "msvc"))]
use jemallocator::Jemalloc;

#[macro_use]
extern crate log;

#[cfg(not(target_env = "msvc"))]
#[global_allocator]
static GLOBAL: Jemalloc = Jemalloc;

mod db;
mod errors;
mod handlers;
mod middleware;
mod routes;
mod schema;
mod utils;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    env_logger::init_from_env(env_logger::Env::default().default_filter_or("debug"));

    const MIGRATIONS: EmbeddedMigrations = embed_migrations!();

    let pool = db::establish_connection();

    HttpServer::new(move || {
        let app_state = AppState::init(pool.clone());

        app_state
            .get_conn()
            .unwrap()
            .run_pending_migrations(MIGRATIONS)
            .unwrap();

        App::new()
            .wrap(Logger::default())
            .app_data(Data::new(app_state))
            .wrap(middleware::cors())
            // .wrap(Compress::default())
            // .wrap(middleware::auth::Authentication)
            .configure(routes::routes)
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}

type PgConn = ConnectionManager<PgConnection>;

#[derive(Clone)]
pub struct AppState {
    pub pool: db::DbPool,
}

impl AppState {
    pub fn get_conn(&self) -> Result<PooledConnection<PgConn>, AppError> {
        let conn = self.pool.get()?;
        Ok(conn)
    }

    pub fn init(pool: Pool<PgConn>) -> Self {
        AppState { pool }
    }
}
