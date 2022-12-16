#[macro_use]
extern crate rocket;
extern crate failure;

pub mod controllers;
pub mod core;
pub mod domain;
pub mod exceptions;
pub mod middlewares;

#[launch]
fn rocket() -> _ {
    core::rocket_factory::build()
}
