use serde::Serialize;

use crate::models::worker_work_details::{PaymentStatus, WorkerWorkDetails};


#[derive(Serialize)]
pub struct WorkerWorkDetailsDTO {
    pub total_lifetime_tasks: i32,
    pub current_no_of_tasks_for_payout: i32,
    pub payout_status: PaymentStatus,
}

impl WorkerWorkDetailsDTO {
    pub fn from(work_details: WorkerWorkDetails) -> Self {
        return WorkerWorkDetailsDTO {
            total_lifetime_tasks: work_details.total_lifetime_tasks,
            current_no_of_tasks_for_payout: work_details.current_no_of_tasks_for_payout,
            payout_status: work_details.payout_status,
        };
    }
}
