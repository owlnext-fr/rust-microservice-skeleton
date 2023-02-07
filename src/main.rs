use crate::core::launcher::{self, Command, CLI};

use anyhow::Result;

use clap::Parser;

#[macro_use]
extern crate rocket;
extern crate anyhow;
extern crate clap;
extern crate diesel_migrations;
pub extern crate validator;

pub mod commands;
pub mod controllers;
pub mod core;
pub mod domain;
pub mod exceptions;
pub mod fixtures;
pub mod libraires;
pub mod middlewares;
pub mod security;

/// main entrypoint of the program.
#[rocket::main]
async fn main() -> Result<()> {
    // parsing CLI class to dispatch action, either trigger the launch
    // of the web API service, or execute a given command.
    let cli = CLI::parse();

    // building the rocket to serve as a pseudo dependency manager
    let rocket = core::rocket_factory::build();

    match &cli.command {
        // launch web API service
        Command::Launch => launcher::engage(rocket).await?,
        // executing given command
        Command::Console { sub_command, args } => launcher::warp(rocket, sub_command, args).await?,
    }

    Ok(())
}
