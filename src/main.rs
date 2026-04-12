use std::{env, sync::Arc};

use axum::{
    Router,
    http::{
        HeaderValue, StatusCode,
        header::{AUTHORIZATION, CONTENT_TYPE},
    },
};
use chrono::ParseError;

use diesel_async::{AsyncPgConnection, pooled_connection::{AsyncDieselConnectionManager, bb8::{Pool, PooledConnection}}};
use tower_http::{
    cors::{Any, CorsLayer},
    trace::TraceLayer,
};
mod models;
mod schema;
mod db;
use tracing_subscriber::{EnvFilter, fmt, layer::SubscriberExt, util::SubscriberInitExt};

use crate::{db::create_db_pool, errors::PersErrors, routes::route_handler::create_route };

mod structs;

mod errors;
mod routes;
mod services;
mod middlewares;
mod utils;
mod dtos;

pub type DbPool =Pool<AsyncPgConnection>;

pub type DbCon<'a> = PooledConnection<'a, AsyncPgConnection>;

#[derive(Debug, Clone)]
pub struct AppState {
    pub db_pool: DbPool,
}



#[tokio::main]
async fn main() -> Result<(), PersErrors> {
    // dotenv setup
    dotenvy::dotenv()
        .map_err(|e| PersErrors::new(e.to_string(), StatusCode::INTERNAL_SERVER_ERROR))?;

    let db_url = env::var("DATABASE_URL")
        .map_err(|e| PersErrors::new(e.to_string(), StatusCode::INTERNAL_SERVER_ERROR))?;
    // pool of db connections
    let db_pool = create_db_pool(db_url).await?;

    // tracng in layerd format
    tracing_subscriber::registry()
        .with(fmt::layer().json()) // show logs in good way
        // RUST_LOG= VALUE DECIDED WHICH ALL LOGS DO WE HAVE TO SEE , INFO ONLY , debug mode
        // debug shows all details , pre api call , api execution details , all
        .with(EnvFilter::try_from_default_env().unwrap_or_else(|_| "info".into()))
        .init();

    // cors
    let cors = CorsLayer::new()
        .allow_methods(Any)
        .allow_origin(
            env::var("FRONTEND_URL")
                .expect("frontend url not defined in env")
                .parse::<HeaderValue>()
                .unwrap(),
        )
        .allow_headers([AUTHORIZATION, CONTENT_TYPE]);

    // 
    let app_state = Arc::new(AppState {
        db_pool,
    });

    // app created
    let app = create_route(Arc::clone(&app_state)).layer(cors);

    println!("app created");

    // genrating address to listen
    let host = env::var("HOST")
        .map_err(|e| PersErrors::new(e.to_string(), StatusCode::INTERNAL_SERVER_ERROR))?;
    let port = env::var("PORT")
        .map_err(|e| PersErrors::new(e.to_string(), StatusCode::INTERNAL_SERVER_ERROR))?;
    let addr = format!("{}:{}", host, port);

    // creating listener and listening to address
    let listener = tokio::net::TcpListener::bind(addr.clone())
        .await
        .map_err(|e| PersErrors::new(e.to_string(), StatusCode::INTERNAL_SERVER_ERROR))?;

    axum::serve(listener, app)
        .await
        .map_err(|e| PersErrors::new(e.to_string(), StatusCode::INTERNAL_SERVER_ERROR))?;

    Ok(())
}
