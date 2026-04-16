use std::sync::Arc;

use axum::{
    Extension, Json, Router,
    extract::Path,
    middleware,
    response::IntoResponse,
    routing::{get, post},
};

use crate::{
    AppState,
    dtos::{
        task_details_with_options_dto::TaskDetailsWithOptionsDTO,
        task_submission_data_dto::{self, TaskSubmissionDataDTO},
        worker_work_details_dto::WorkerWorkDetailsDTO,
    },
    errors::PersErrors,
    middlewares::{
        auth_middleware::authenticate_user, user_type_middleware::worker_validator_middleware,
    },
    services::worker::{new_task_for_worker, worker_dashboard_data, worker_task_submission},
    structs::EncodedUserData,
};

// worker middleware with user_auth + user_type = WORKER MIDDLEWARE CONSTRINTS
pub fn worker_routes() -> Router {
    Router::new()
        .route("/next-task", get(show_next_unique_task))
        .route("/submit-task", post(submit_worker_response))
        .route("/worker-dashboard-details", get(worker_dashboard_details))
        .layer(middleware::from_fn(worker_validator_middleware))
        .layer(middleware::from_fn(authenticate_user))
}

/**
 * constraints => verified user + user type=> WORKER (validated by AUTH + USER_TYPE middleware)
 * @inputs => user wallet ddress from the token
 * @what_we_are_doing =>
 *
 *      we will return user a new task that user hasnt done yet
 *
 */
pub async fn show_next_unique_task(
    Extension(app_state): Extension<Arc<AppState>>,
    Extension(user_details): Extension<EncodedUserData>,
) -> Result<impl IntoResponse, PersErrors> {
    let db_pool = app_state.db_pool.clone();

    let new_task_for_worker =
        new_task_for_worker(&db_pool, user_details.user_wallet_address).await?;
    Ok(Json(TaskDetailsWithOptionsDTO::from(new_task_for_worker)))
}

// submit a reponse of a task submitted by a user
pub async fn submit_worker_response(
    Extension(app_state): Extension<Arc<AppState>>,
    Extension(user_details): Extension<EncodedUserData>,
    Json(task_submission_data): Json<TaskSubmissionDataDTO>,
) -> Result<impl IntoResponse, PersErrors> {
    let db_pool = app_state.db_pool.clone();

    worker_task_submission(
        &db_pool,
        user_details.user_wallet_address,
        task_submission_data.task_id,
        task_submission_data.selection_option_id,
    )
    .await?;

    Ok("Submitted".to_string())
}

/**
 * worker dashboard details api
 */
pub async fn worker_dashboard_details(
    Extension(app_state): Extension<Arc<AppState>>,
    Extension(user_details): Extension<EncodedUserData>,
) -> Result<impl IntoResponse, PersErrors> {
    let db_pool = app_state.db_pool.clone();

    let details = worker_dashboard_data(&db_pool, user_details.user_wallet_address).await?;

    Ok(Json(WorkerWorkDetailsDTO::from(details)))
}

/**
 * worker will presee payout button and we will check
 * we will check status of payout , not eligible eligible , locked(in processing , can withdraw)
 * 1) is user eligible/can_withdraw for payout (minimum amount > 10$ , that is no_of_currnt_task*0.05$)
 *      if status locked , then we will check for txn status
 *      if txn fails we rery or put the stast to same state
 *      if completed then we will update the details
 *
 * 2) user bolega muje mere paise dedo , so we will trasnfer money from our wallet to user address
 * 3) a txn will be generated we will create a payout details in payout table, that txn can be in pending state , so if pending , we will save txn hash and put payout in pending
 */
pub async fn worker_payout() -> Result<(), PersErrors> {
    Ok(())
}
