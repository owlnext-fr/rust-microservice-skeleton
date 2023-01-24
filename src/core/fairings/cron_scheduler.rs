use rocket::{
    fairing::{Fairing, Info, Kind},
    Orbit, Rocket,
};
use tokio_cron_scheduler::Job;

use crate::core::commands::{
    command::{Command, CommandHandle},
    scheduler_builder::SchedulerBuilder,
};

/// a rocket fairing enabling async tasks (eg crons) while rocket is launching
#[derive(Default)]
pub struct CronScheduler {
    crons: Vec<CommandHandle<dyn Command>>,
}

impl CronScheduler {
    /// adds a cron (eg CommandHandle with a given command) to run with the scheduler.
    pub fn add_cron(&mut self, cron: CommandHandle<dyn Command>) {
        self.crons.push(cron);
    }
}

#[rocket::async_trait]
impl Fairing for CronScheduler {
    fn info(&self) -> Info {
        Info {
            name: "Cron scheduler",
            kind: Kind::Liftoff,
        }
    }

    async fn on_liftoff(&self, _rocket: &Rocket<Orbit>) {
        let sched = SchedulerBuilder::build().await;

        for handle in self.crons.iter() {
            let schedule = handle.schedule.clone();

            let handle = handle.clone();
            debug!("{schedule}");
            let job = Job::new_cron_job_async(schedule.as_str(), move |_uid, _lock| {
                let handle = handle.clone();

                Box::pin(async move {
                    handle.command.run().await.unwrap();
                })
            })
            .unwrap();

            sched.add(job).await.unwrap();
        }

        sched.start().await.unwrap();
    }
}
