use tokio_cron_scheduler::JobScheduler;

pub struct SchedulerBuilder {}

impl SchedulerBuilder {
    pub async fn build() -> JobScheduler {
        let mut sched = JobScheduler::new().await.unwrap();

        sched.shutdown_on_ctrl_c();

        sched
    }
}
