use router::create_router;

mod api;
mod error;
mod router;
mod todo;

fn init_tracing() {
    use tracing_subscriber::{EnvFilter, filter::LevelFilter, fmt, prelude::*};

    let rust_log = std::env::var(EnvFilter::DEFAULT_ENV)
        .unwrap_or_else(|_| "sqlx=info,tower_http=info,info".to_string());

    tracing_subscriber::registry()
        .with(fmt::layer())
        .with(
            EnvFilter::builder()
                .with_default_directive(LevelFilter::INFO.into())
                .parse_lossy(rust_log),
        )
        .init();
}

async fn init_db_pool() -> Result<sqlx::Pool<sqlx::Sqlite>, sqlx::Error> {
    use sqlx::sqlite::{SqliteConnectOptions, SqlitePoolOptions};
    use std::str::FromStr;

    let db_connection_str =
        std::env::var("DATABASE_URL").unwrap_or_else(|_| "sqlite:db.sqlite".to_string());

    let db_pool = SqlitePoolOptions::new()
        .connect_with(SqliteConnectOptions::from_str(&db_connection_str)?.create_if_missing(true))
        .await
        .expect("can't connect to database");

    sqlx::migrate!("./migrations/")
        .run(&db_pool)
        .await
        .expect("database migration failed");

    Ok(db_pool)
}

#[tokio::main]
async fn main() {
    init_tracing();

    let db_pool = init_db_pool()
        .await
        .expect("can't initialize database pool");
    let router = create_router(db_pool).await;
    let bind_addr = std::env::var("BIND_ADDR").unwrap_or_else(|_| "127.0.0.1:3000".to_string());

    let listener = tokio::net::TcpListener::bind(&bind_addr)
        .await
        .expect("can't bind to address");

    axum::serve(listener, router)
        .await
        .expect("can't start server");
}
