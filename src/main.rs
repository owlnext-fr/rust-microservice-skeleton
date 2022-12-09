use base::rocket_factory;

#[macro_use]
extern crate rocket;

pub mod base;
pub mod controllers;
pub mod exceptions;

#[launch]
fn rocket() -> _ {
    rocket_factory::build()
}
