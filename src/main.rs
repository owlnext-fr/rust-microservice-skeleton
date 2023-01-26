#[macro_use]
extern crate rocket;
extern crate anyhow;
extern crate diesel_migrations;
extern crate failure;

pub mod commands;
pub mod controllers;
pub mod core;
pub mod domain;
pub mod exceptions;
pub mod fixtures;
pub mod middlewares;
pub mod security;

#[launch]
fn launch() -> _ {
    core::rocket_factory::build()
}
