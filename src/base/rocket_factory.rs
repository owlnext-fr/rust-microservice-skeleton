use rocket::{Build, Rocket};

use crate::controllers::{app, catchers};

use super::configuration;

pub fn build() -> Rocket<Build> {
    rocket::build()
        // routes
        .mount("/", routes![app::index::index])
        // catchers
        .register("/", catchers![catchers::default_catcher])
        // middlewares/fairings
        .manage(configuration::load())
}
