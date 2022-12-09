use rocket::{Build, Rocket};

use super::{configuration, fairings::jwt_certificates::JWTCertificatesFairing};
use crate::controllers::{app, catchers};

pub fn build() -> Rocket<Build> {
    rocket::build()
        // routes
        .mount("/", routes![app::index::index])
        // catchers
        .register("/", catchers![catchers::default_catcher])
        // middlewares/fairings
        .manage(configuration::load())
        // fairings
        .attach(JWTCertificatesFairing::default())
}
