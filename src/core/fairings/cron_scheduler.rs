use std::time::Duration;

use rocket::{
    fairing::{Fairing, Info, Kind},
    Orbit, Rocket,
};
use tokio_cron_scheduler::Job;

use crate::core::commands::scheduler_builder::SchedulerBuilder;

#[derive(Default)]
pub struct CronScheduler {}

#[rocket::async_trait]
impl Fairing for CronScheduler {
    fn info(&self) -> Info {
        Info {
            name: "Cron scheduler",
            kind: Kind::Liftoff,
        }
    }

    async fn on_liftoff(&self, rocket: &Rocket<Orbit>) {

        // let mut sched = SchedulerBuilder::build().await;

        // let jja = Job::new_repeated_async(Duration::from_secs(7), |_uuid, _l| {
        //     Box::pin(async move {
        //         println!(
        //             "{:?} I'm repeated async every 7 seconds",
        //             chrono::Utc::now()
        //         );
        //     })
        // })
        // .unwrap();

        // sched.add(jja).await;

        // sched.start().await;
    }
}
