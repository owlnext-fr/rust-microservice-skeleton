use super::{
    commands::console_command_registry::ConsoleCommandRegistry,
    configuration::ConfigState,
    database::{get_connection_pool, DbPoolState},
    fairings::{
        database_migrations::DatabaseMigrations, fixture::FixtureLoader,
        jwt_certificates::JWTCertificatesFairing,
    },
    security::{Security, SecurityVoter},
};
use crate::{
    commands::test_command::TestCommand,
    controllers::api::{account, auth, security_test},
    domain::repository::{
        account_repository::AccountRepository, application_repository::ApplicationRepository,
        cron_log_repository::CronLogRepository, refresh_token_repository::RefreshTokenRepository,
        user_repository::UserRepository,
    },
    middlewares::{
        account_middleware::AccountMiddleware, application_middleware::ApplicationMiddleware,
        cron_log_middleware::CronLogMiddleware, refresh_token_middleware::RefreshTokenMiddleware,
        user_middleware::UserMiddleware,
    },
};
use crate::{controllers::app, security::handlers::account_security::AccountSecurityVoter};
use crate::{core::catcher, security::handlers::test_security::TestSecurityVoter};
use rocket::{Build, Rocket};
use std::sync::Arc;

#[allow(clippy::redundant_clone, unused_mut)]
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
    // -- command registry initialization --
    //
    #[allow(unused_mut)]
    let mut command_registry = ConsoleCommandRegistry::new();

    //
    // -- command registry insertions --
    //
    command_registry.add(Arc::new(TestCommand::new(cron_log_middleware.clone())));

    //
    // -- security --
    //
    let mut security = Security::<dyn SecurityVoter>::new();
    security.add_voter(Box::<AccountSecurityVoter>::default());

    //
    // -- fixtures --
    //
    let mut fixture_loader = FixtureLoader::default();
    // fixture_loader.add_fixture(Arc::new(...));

    //
    // -- starting rocket setup --
    //
    let mut build = rocket::build();

    //
    // -- starting rocket setup --
    //
    if configuration.get_string_or_default("env", "dev") == "dev" {
        build = build
            // security testing routes
            .mount(
                "/api/security-test",
                routes![security_test::test_connected, security_test::test_secured],
            );

        // security testing voter
        security.add_voter(Box::<TestSecurityVoter>::default());
    }

    build = build
        // routes
        .mount("/", routes![app::index::index])
        .mount("/api/auth", routes![auth::token, auth::refresh_token])
        .mount(
            "/api",
            routes![account::account_list, account::account_details],
        )
        // catchers
        .register("/", catchers![catcher::default_catcher])
        // managed global states
        .manage(configuration)
        .manage(db_state)
        .manage(security)
        .manage(command_registry)
        // managed middlewares
        .manage(user_middleware)
        .manage(refresh_token_middleware)
        .manage(cron_log_middleware)
        .manage(application_middleware)
        .manage(account_middleware)
        // fairings
        .attach(DatabaseMigrations::default())
        .attach(JWTCertificatesFairing::default())
        .attach(fixture_loader);

    build
}
