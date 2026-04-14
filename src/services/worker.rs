use axum::http::StatusCode;
use diesel::{
    ExpressionMethods, QueryDsl, SelectableHelper,
    dsl::{exists, not, sql},
    sql_types::Text,
};
use diesel_async::{AsyncConnection, RunQueryDsl, scoped_futures::ScopedFutureExt};
use uuid::Uuid;

use crate::{
    DbPool,
    db::get_connection_from_pool,
    errors::PersErrors,
    models::{
        submission_details::{NewSubmissionDetails, SubmissionDetails},
        task::{Task, TaskUpdates},
        task_options::TaskOption,
        worker_work_details::WorkerWorkDetails,
    },
    schema::{
        sql_types::TaskStatus,
        submission_details,
        task_options::{self, no_of_times_image_selected},
        tasks::{self, is_active, no_of_times_taks_done},
        worker_work_details::{self, current_no_of_tasks_for_payout, total_lifetime_tasks},
    },
    services::tasks::get_creator_task_details,
};

/**
 * @inputs => db_pool and user wallet address
 * @result => return the worker a new task (that he hasnt done yet)
 */
pub async fn new_task_for_worker(
    db_pool: &DbPool,
    user_wallet_address: String,
) -> Result<(Task, Vec<TaskOption>), PersErrors> {
    let mut db_con = get_connection_from_pool(db_pool).await?;

    // we will find a task from task table
    // where taskid is not present in submissions

    // unique task id that user hasnt done yet
    //  using not exist
    // we are taking 1 task from task table , and running query does this task id present in submission table (where user id is this and task id is this )
    // better that not in and leftjoin (intermediate state would be very big and they will find all and then return )
    // while not exist will find as much as required and return early
    let task = tasks::table
        .filter(not(exists(
            submission_details::table
                .filter(submission_details::worker_wallet_address.eq(user_wallet_address))
                .filter(submission_details::task_id.eq(tasks::id)),
        )))
        .filter(tasks::no_of_times_taks_done.lt(1000))
        .limit(1)
        .first::<Task>(&mut db_con)
        .await
        .map_err(|e| match e {
            diesel::result::Error::NotFound => PersErrors::new(
                format!("No task found for worker => {}", e.to_string()),
                StatusCode::INTERNAL_SERVER_ERROR,
            ),
            _ => PersErrors::new(
                format!(
                    "Error while finding unique task for worker => {}",
                    e.to_string()
                ),
                StatusCode::INTERNAL_SERVER_ERROR,
            ),
        })?;

    let task_options = get_creator_task_details(task.id.to_owned(), &db_pool).await?;

    Ok((task, task_options))
}

/**
 * @inputs => db_pool and user wallet address , task id , and option which user has selected
 * @result => updating all these in transaction
 *  // 1) no_of_times_taks_done for task 
    // 2) create a new submission 
    // 3) update worker work details 
    // 4) task_option => no of times this option selected count +1
 */
pub async fn worker_task_submission(
    db_pool: &DbPool,
    user_wallet_address: String,
    task_id: Uuid,
    task_option_selected: Uuid,
) -> Result<(), PersErrors> {
    

    let mut db_con = get_connection_from_pool(db_pool).await?;

    db_con
        .transaction::<_, diesel::result::Error, _>(|db_con| {
            async move {
                // updating task and also validating that task_suubmission should be less that 1000
                // and also if task no of time == 1000 , then setting the is_active to completed
                // done in 1 query only becasue , if we first do and then later update then it may be possible
                // that other user also updating after this , so it may crete race condiion
                diesel::update(
                    tasks::table
                        .filter(tasks::id.eq(task_id.to_owned()))
                        .filter(tasks::no_of_times_taks_done.lt(1000))
                        .filter(tasks::is_active.eq(TaskUpdates::ACTIVE)),
                )
                .set((
                    tasks::no_of_times_taks_done.eq(no_of_times_taks_done + 1),
                    tasks::is_active.eq(sql::<TaskStatus>(
                        "CASE
                        WHEN no_of_times_taks_done +1 = 1000 THEN 'COMPLETED'
                        ELSE is_active
                        END",
                    )),
                ))
                .get_result::<Task>(db_con)
                .await?;

                // submitting a new task by the worker
                let new_submission = NewSubmissionDetails {
                    worker_wallet_address: user_wallet_address.to_owned(),
                    task_id: task_id,
                    selected_option_id: task_option_selected,
                };

                let sub = diesel::insert_into(submission_details::table)
                    .values(new_submission)
                    .returning(SubmissionDetails::as_returning())
                    .get_result(db_con)
                    .await?;

                let updated_worker = diesel::update(worker_work_details::table.filter(
                    worker_work_details::worker_wallet_address.eq(user_wallet_address.to_owned()),
                ))
                .set((
                    worker_work_details::total_lifetime_tasks.eq(total_lifetime_tasks + 1),
                    worker_work_details::current_no_of_tasks_for_payout
                        .eq(current_no_of_tasks_for_payout + 1),
                ))
                .get_result::<WorkerWorkDetails>(db_con)
                .await?;

                let options = diesel::update(
                    task_options::table
                        .filter(task_options::id.eq(task_option_selected.to_owned())),
                )
                .set(task_options::no_of_times_image_selected.eq(no_of_times_image_selected + 1))
                .get_result::<TaskOption>(db_con)
                .await?;

                Ok(())
            }
            .scope_boxed()
        })
        .await
        .map_err(|e| match e {
            diesel::result::Error::NotFound => {
                // not found means submission task is wrong , or task is already complete for which req is made
                PersErrors::new(e.to_string(), StatusCode::CONFLICT)
            }
            _ => PersErrors::new(e.to_string(), StatusCode::INTERNAL_SERVER_ERROR),
        })?;

    Ok(())
}

pub async fn worker_dashboard_data(
    db_pool: &DbPool,
    user_wallet_address: String,
) -> Result<WorkerWorkDetails, PersErrors> {
    let mut db_con = get_connection_from_pool(db_pool).await?;

    let worker_details = worker_work_details::table
        .filter(worker_work_details::worker_wallet_address.eq(user_wallet_address.to_owned()))
        .first::<WorkerWorkDetails>(&mut db_con)
        .await
        .map_err(|e| {
            PersErrors::new(
                format!(
                    "error fetching worker dashboard details {}: {}",
                    user_wallet_address, e
                ),
                StatusCode::INTERNAL_SERVER_ERROR,
            )
        })?;

    Ok(worker_details)
}
