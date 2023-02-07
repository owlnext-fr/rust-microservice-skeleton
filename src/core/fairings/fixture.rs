use std::sync::Arc;

use rocket::{
    fairing::{Fairing, Info, Kind},
    Orbit, Rocket,
};

use anyhow::Result;

/// A behaviour for static/dynamic data fixtures launched at runtime.
pub trait Fixture: Send + Sync {
    /// checks if the current fixture must loads its data.
    /// This is the method to use if you need your data to load only once.
    ///
    /// It must return `true` if the data needs to be loaded, `false` otherwise.
    fn must_load(&self) -> bool;
    /// entrypoint of the fixture ; loads the data.
    fn load(&self) -> Result<()>;
}

/// A struct acting as a registry of all data fixtures for the application.
#[derive(Default)]
pub struct FixtureLoader {
    /// the actual registry.
    fixtures: Vec<Arc<dyn Fixture>>,
}

impl FixtureLoader {
    /// ads a fixture to the registry.
    ///
    /// **Note:** the fixture instance must be Arc'ed.
    pub fn add_fixture(&mut self, fixture: Arc<dyn Fixture>) {
        self.fixtures.push(fixture);
    }

    /// this will trigger the registry to pass over all the fixtures contained, and try to load them if needed.
    pub fn run(&self) -> Result<()> {
        for fixture in self.fixtures.iter() {
            if fixture.must_load() {
                fixture.load()?;
            }
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

    /// fairing event to run all fixtures if needed upon liftoff
    async fn on_liftoff(&self, _rocket: &Rocket<Orbit>) {
        self.run().unwrap();
    }
}
