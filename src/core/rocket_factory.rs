use std::sync::Arc;

use rocket::{Build, Rocket};

use super::{
    configuration::ConfigState,
    database::{get_connection_pool, DbPoolState},
    fairings::{
        cron_scheduler::CronScheduler, database_migrations::DatabaseMigrations,
        fixture::FixtureLoader, jwt_certificates::JWTCertificatesFairing,
    },
    security::{Security, SecurityVoter},
};
use crate::{
    controllers::{
        api::{auth, security_test},
        app, catchers,
    },
    domain::repository::{
        account_repository::AccountRepository, application_repository::ApplicationRepository,
        cron_log_repository::CronLogRepository, refresh_token_repository::RefreshTokenRepository,
        user_repository::UserRepository,
    },
    fixtures::init_fixture::InitFixture,
    middlewares::{
        account_middleware::AccountMiddleware, application_middleware::ApplicationMiddleware,
        cron_log_middleware::CronLogMiddleware, refresh_token_middleware::RefreshTokenMiddleware,
        user_middleware::UserMiddleware,
    },
    security::handlers::test_security_handler::TestSecurityHandler,
};

#[allow(clippy::redundant_clone)]
pub fn build() -> Rocket<Build> {
    //
    // -- configuration initialisation --
    //
    let configuration = ConfigState::load();

    //
    // -- database initialisation --
    //
    let db_conn_url = configuration.get_string("database_url").unwrap();
    let db_pool = get_connection_pool(db_conn_url).unwrap();
    let db_state = DbPoolState { db_pool };

    //
    // -- repository initialisation --
    //
    let user_rep = UserRepository::new(db_state.clone());
    let refresh_token_rep = RefreshTokenRepository::new(db_state.clone());
    let cron_log_rep = CronLogRepository::new(db_state.clone());
    let application_rep = ApplicationRepository::new(db_state.clone());
    let account_rep = AccountRepository::new(db_state.clone());

    //
    // -- middleware initialisation --
    //
    let user_middleware = UserMiddleware::new(user_rep.clone(), configuration.clone());
    let refresh_token_middleware =
        RefreshTokenMiddleware::new(refresh_token_rep.clone(), configuration.clone());
    let cron_log_middleware = CronLogMiddleware::new(cron_log_rep.clone());
    let application_middleware = ApplicationMiddleware::new(application_rep.clone());
    let account_middleware = AccountMiddleware::new(account_rep.clone());

    //
    // -- scheduler initialisation --
    //
    #[allow(unused_mut)]
    let mut sched = CronScheduler::default();

    //
    // -- scheduler setup --
    //
    // sched.add_cron(CommandHandle {
    //     command: Arc::new(TestCommand {
    //         name: "app:test".to_string(),
    //         args: None,
    //         cron_log_middleware: cron_log_middleware.clone(),
    //     }),
    //     schedule: "*/5 * * * * * *".to_string(),
    // });

    //
    // -- security --
    //
    let mut security = Security::<dyn SecurityVoter>::new();

    //
    // -- fixtures --
    //
    let mut fixture_loader = FixtureLoader::default();

    fixture_loader.add_fixture(Arc::new(InitFixture::new(
        account_middleware.clone(),
        application_middleware.clone(),
        user_middleware.clone(),
        configuration.clone(),
    )));

    //
    // -- starting rocket setup --
    //
    let mut build = rocket::build();

    //
    // -- starting rocket setup --
    //
    if configuration.get_string_or_default("env", "dev") == "dev" {
        build = build
            // catcher for invalid JSON input
            .register("/", catchers![rocket_validation::validation_catcher])
            // security testing routes
            .mount(
                "/api/security-test",
                routes![security_test::test_connected, security_test::test_secured],
            );

        // security testing voter
        security.add_handler(Box::new(TestSecurityHandler::default()));
    }

    build = build
        // routes
        .mount("/", routes![app::index::index])
        .mount("/api/auth", routes![auth::token, auth::refresh_token])
        // catchers
        .register("/", catchers![catchers::default_catcher])
        // managed global states
        .manage(configuration)
        .manage(db_state)
        .manage(security)
        // managed middlewares
        .manage(user_middleware)
        .manage(refresh_token_middleware)
        .manage(cron_log_middleware)
        .manage(application_middleware)
        .manage(account_middleware)
        // fairings
        .attach(DatabaseMigrations::default())
        .attach(JWTCertificatesFairing::default())
        .attach(sched)
        .attach(fixture_loader);

    build
}
