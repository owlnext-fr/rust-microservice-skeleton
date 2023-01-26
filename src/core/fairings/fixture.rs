use std::sync::Arc;

use rocket::{
    fairing::{Fairing, Info, Kind},
    Orbit, Rocket,
};

use anyhow::Result;

pub trait Fixture: Send + Sync {
    fn load(&self) -> Result<()>;
}

#[derive(Default)]
pub struct FixtureLoader {
    fixtures: Vec<Arc<dyn Fixture>>,
}

impl FixtureLoader {
    pub fn add_fixture(&mut self, fixture: Arc<dyn Fixture>) {
        self.fixtures.push(fixture);
    }

    pub fn run(&self) -> Result<()> {
        for fixture in self.fixtures.iter() {
            debug!("fixture launching!");
            fixture.load()?;
            debug!("fixture finished!");
        }

        Ok(())
    }
}

#[rocket::async_trait]
impl Fairing for FixtureLoader {
    fn info(&self) -> Info {
        Info {
            name: "Fixture loader",
            kind: Kind::Liftoff,
        }
    }

    async fn on_liftoff(&self, _rocket: &Rocket<Orbit>) {
        self.run().unwrap();
    }
}
