use std::sync::Arc;

use axum::{Extension, http::StatusCode};

use diesel::{ExpressionMethods, QueryDsl, SelectableHelper};
use diesel_async::{AsyncConnection, RunQueryDsl, scoped_futures::ScopedFutureExt};
use uuid::Uuid;

use crate::{
    AppState, DbPool,
    db::get_connection_from_pool,
    dtos::new_task_dto::NewTaskDTO,
    errors::PersErrors,
    models::{
        task::{NewTask, Task, TaskUpdates},
        task_options::{NewTaskOption, TaskOption},
    },
    schema::{task_options, tasks},
};

/**
 * @inputs => user_id and new task details
 * @what_we_are_doing =>
 * 1) 1 transaction (create new task + store all image options)
 * 2)
 */
pub async fn create_new_task(
    db_pool: &DbPool,
    user_wallet_address: String,
    new_task_details: NewTaskDTO,
) -> Result<Uuid, PersErrors> {
    let mut conn = get_connection_from_pool(db_pool).await?;

    let user_wallet_address = user_wallet_address.clone();
    let txn_sign = new_task_details.txn_sign.clone();

    println!("we are just before the transaction");

    let task_id = conn.transaction::<_, diesel::result::Error, _>(|conn| {
        async move {
            // creating new task
            let new_task = NewTask {
                user_wallet_address,
                txn_hash: txn_sign,
                is_active: TaskUpdates::ACTIVE,
                no_of_times_taks_done: 0,
            };
            // adding new task to db
            let task = diesel::insert_into(tasks::table)
                .values(&new_task)
                .returning(Task::as_returning())
                .get_result::<Task>(conn)
                .await?;

            // storing all image options in the db
            let mut vec_of_new_taks_options: Vec<NewTaskOption> = Vec::new();

            for img_url in new_task_details.images {
                let new_option = NewTaskOption {
                    task_id: task.id.to_owned(),
                    image_url: img_url,
                    no_of_times_image_selected: 0,
                };
                vec_of_new_taks_options.push(new_option);
            }

            let res = diesel::insert_into(task_options::table)
                .values(vec_of_new_taks_options)
                .returning(TaskOption::as_returning())
                .get_result::<TaskOption>(conn)
                .await?;

            println!("these are all the options {:?}", res);

            Ok(task.id)
        }
        .scope_boxed()
    })
    .await
    .map_err(|e| PersErrors::new(e.to_string(), StatusCode::INTERNAL_SERVER_ERROR))?;

    Ok(task_id)
}

/**
* @inputs => dbpool and user_Wallet_Address
* @result => we will return this creators all taks
*/
pub async fn get_creator_all_task(
    db_pool: &DbPool,
    user_wallet_address: String,
) -> Result<Vec<Task>, PersErrors> {
    let mut db_con = get_connection_from_pool(db_pool).await?;

    let creator_all_tasks = tasks::table
        .filter(tasks::user_wallet_address.eq(user_wallet_address))
        .select(Task::as_select())
        .load::<Task>(&mut db_con)
        .await
        .map_err(|e| PersErrors::new(e.to_string(), StatusCode::INTERNAL_SERVER_ERROR))?;

    Ok(creator_all_tasks)
}

/**
 * @inputs => task id (of which we want details)
 * @result => we will return task options details
 */
pub async fn get_creator_task_details(task_id: Uuid, db_pool: &DbPool) -> Result< Vec<TaskOption>, PersErrors> {
    let mut db_con = get_connection_from_pool(db_pool).await?;

    let task_options = task_options::table
        .filter(task_options::task_id.eq(task_id))
        .select(TaskOption::as_select())
        .load(&mut db_con)
        .await
        .map_err(|e| PersErrors::new(e.to_string(), StatusCode::INTERNAL_SERVER_ERROR))?;

    Ok(task_options)
}
