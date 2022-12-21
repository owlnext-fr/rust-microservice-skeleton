#[macro_use]
extern crate rocket;
extern crate failure;

pub mod controllers;
pub mod core;
pub mod domain;
pub mod exceptions;
pub mod middlewares;
pub mod security;
#[launch]
fn rocket() -> _ {
    core::rocket_factory::build()
}
