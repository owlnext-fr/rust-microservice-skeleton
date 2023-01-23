use crate::core::configuration::ConfigState;
use crate::diesel_migrations::MigrationHarness;
use diesel::{Connection, PgConnection};
use diesel_migrations::{embed_migrations, EmbeddedMigrations};
use rocket::{
    fairing::{Fairing, Info, Kind},
    log::private::debug,
    Orbit, Rocket,
};

#[derive(Default)]
pub struct DatabaseMigrations {}

#[rocket::async_trait]
impl Fairing for DatabaseMigrations {
    fn info(&self) -> Info {
        Info {
            name: "Database migrations",
            kind: Kind::Liftoff,
        }
    }

    async fn on_liftoff(&self, rocket: &Rocket<Orbit>) {
        let configuration = rocket.state::<ConfigState>().unwrap();
        let conn_url = configuration.get_string("database_url").unwrap();

        let mut conn = PgConnection::establish(&conn_url).unwrap();

        pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!();

        conn.run_pending_migrations(MIGRATIONS).unwrap();

        debug!(" >> Migration done !");
    }
}
