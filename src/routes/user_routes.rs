use axum::{Router, routing::get};
use tracing::info;



pub fn user_handler()-> Router{
    Router::new()
        .route("/check", get(verify))
}


pub async fn verify(){

    tracing::info!("it is working bruhh");


}