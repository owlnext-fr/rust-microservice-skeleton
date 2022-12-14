use rocket::{Build, Rocket};

use super::{
    configuration,
    database::{get_connection_pool, DbPoolState},
    fairings::jwt_certificates::JWTCertificatesFairing,
};
use crate::controllers::{api::auth, app, catchers};

pub fn build() -> Rocket<Build> {
    let configuration = configuration::load();

    let db_pool = get_connection_pool(configuration.get_string("database_url").unwrap()).unwrap();

    let mut build = rocket::build();

    if configuration
        .get_string("env")
        .unwrap_or_else(|_| String::from("prod"))
        == "dev"
    {
        build = build.register("/", catchers![rocket_validation::validation_catcher]);
    }

    build = build
        // routes
        .mount("/", routes![app::index::index])
        .mount("/api/auth", routes![auth::token])
        // catchers
        .register("/", catchers![catchers::default_catcher])
        // managed states
        .manage(configuration)
        .manage(DbPoolState { db_pool })
        // fairings
        .attach(JWTCertificatesFairing::default());

    build
}
