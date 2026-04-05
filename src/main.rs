use std::{env, sync::Arc};

use axum::{
    Router,
    http::{
        HeaderValue, StatusCode,
        header::{AUTHORIZATION, CONTENT_TYPE},
    },
};
use tower_http::{
    cors::{Any, CorsLayer},
    trace::TraceLayer,
};
mod models;
mod schema;
use tracing_subscriber::{EnvFilter, fmt, layer::SubscriberExt, util::SubscriberInitExt};

use crate::{errors::PersErrors, routes::route_handler::create_route};

mod errors;
mod routes;

#[derive(Debug, Clone)]
pub struct AppState {
    pub db_config: String,
}

#[tokio::main]
async fn main() -> Result<(), PersErrors> {
    // dotenv setup
    dotenvy::dotenv()
        .map_err(|e| PersErrors::new(e.to_string(), StatusCode::INTERNAL_SERVER_ERROR))?;

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

    // no need mutex as of now , becasue db_config can be borrowed , no changing , so all okay
    let app_state = Arc::new(AppState {
        db_config: "dummy".to_string(),
    });

    // app created
    let app = create_route(Arc::clone(&app_state)).layer(cors);

    println!("app created");

    let host = env::var("HOST").map_err(|e| PersErrors::new(e.to_string(), StatusCode::INTERNAL_SERVER_ERROR))?;
    let port = env::var("PORT").map_err(|e| PersErrors::new(e.to_string(), StatusCode::INTERNAL_SERVER_ERROR))?;

    let addr = format!("{}:{}" , host , port);

    let listener = tokio::net::TcpListener::bind(addr.clone())
        .await
        .map_err(|e| PersErrors::new(e.to_string(), StatusCode::INTERNAL_SERVER_ERROR))?;
    
    tracing::info!("{}" , format!(" listening at {}" , addr));
    axum::serve(listener, app)
        .await
        .map_err(|e| PersErrors::new(e.to_string(), StatusCode::INTERNAL_SERVER_ERROR))?;


    

    Ok(())
}
