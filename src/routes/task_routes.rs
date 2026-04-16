use std::sync::Arc;

use axum::{
    Extension, Json, Router, extract::Path, middleware, response::IntoResponse, routing::{get, post}
};
use uuid::Uuid;

use crate::{
    AppState,
    dtos::{new_task_dto::NewTaskDTO, task_dto::TaskDTO, task_id_dto::TaskIdDTO},
    errors::PersErrors,
    middlewares::{
        auth_middleware::authenticate_user, user_type_middleware::creator_validator_middleware,
    },
    models::{task::Task, task_options::TaskOption},
    services::tasks::{create_new_task, get_creator_all_task, get_creator_task_details},
    structs::EncodedUserData,
};

pub fn task_handler() -> Router {
    Router::new()
        
        .merge(creators_routes())
}

// routes which requires existing user + user=creator
pub fn creators_routes() -> Router {
    Router::new()
        .route("/create-new-task", post(create_task))
        .route("/get-task/{task_id}", get(get_task_details))
        .route("/get-all-my-tasks", get(get_all_my_task)) // then the req comes to handlers
        .layer(middleware::from_fn(creator_validator_middleware)) // secondly time it goes here
        .layer(middleware::from_fn(authenticate_user)) // first req will go to this
}

/**
 * constraints => verified user + user type=> CREATOR (validated by AUTH + USER_TYPE middleware)
 * @inputs => payment hash + aws stored images array
 * @what_we_are_doing =>
 *
 * 1) validating txn (user has actually paid the amount)
 * 2) and we will create task + options
 *
 */
pub async fn create_task(
    Extension(user_data): Extension<EncodedUserData>,
    Extension(app_state): Extension<Arc<AppState>>,
    Json(task_details): Json<NewTaskDTO>,
) -> Result<impl IntoResponse, PersErrors> {
    // validate txn hash

    // create a task and store images in options (a transction call to store data in multiple tables )

    let db_pool = app_state.db_pool.clone();

    // create new tasks and options
    let task_id = create_new_task(
        &db_pool,
        user_data.user_wallet_address.to_owned(),
        task_details,
    )
    .await?;

    Ok(Json(TaskIdDTO{
        task_id : task_id.to_string()
    }))
}

/**
 * write accordingly
 * constraints =>
 * @inputs => payment hash + aws stored images array
 * @what_we_are_doing =>
 *
 * 1) validating txn (user has actually paid the amount)
 * 2) and we will create task + options
 *
 */
pub async fn get_task_details(
    Extension(user_details): Extension<EncodedUserData>,
    Extension(app_state): Extension<Arc<AppState>>,
    Path(task_id) : Path<Uuid>
) -> Result<impl IntoResponse, PersErrors> {

    println!("user , task id = {user_details:?} {task_id}");

    let db_pool = app_state.db_pool.clone();
    let task_images = get_creator_task_details(task_id, &db_pool).await?;
    Ok(Json(task_images))
}

/**
 * constraints => auth + user_type = Creator (middleware)
 * @inputs => from middleware , we will get user wallet address , and hence we will return creator all tasks he creatd till now
 * @what_we_are_doing =>
 *
 *  just returning all the tasks
 */
pub async fn get_all_my_task(
    Extension(user_details): Extension<EncodedUserData>,
    Extension(app_state): Extension<Arc<AppState>>,
) -> Result<impl IntoResponse, PersErrors> {
    let db_pool = app_state.db_pool.clone();

    let user_wallet_address = user_details.user_wallet_address;

    let all_tasks = get_creator_all_task(&db_pool, user_wallet_address).await?;

    let vec_of_task_dto = TaskDTO::to_tasks(all_tasks);
    Ok(Json(vec_of_task_dto))
}
