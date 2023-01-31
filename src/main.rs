use crate::core::launcher::{self, Command, CLI};

use anyhow::Result;

use clap::Parser;

#[macro_use]
extern crate rocket;
extern crate anyhow;
extern crate clap;
extern crate diesel_migrations;
extern crate failure;
pub extern crate validator;

pub mod commands;
pub mod controllers;
pub mod core;
pub mod domain;
pub mod exceptions;
pub mod fixtures;
pub mod middlewares;
pub mod security;

#[rocket::main]
async fn main() -> Result<()> {
    let cli = CLI::parse();

    let rocket = core::rocket_factory::build();

    match &cli.command {
        Command::Launch => launcher::engage(rocket).await?,
        Command::Console { sub_command, args } => launcher::warp(rocket, sub_command, args).await?,
    }

    Ok(())
}
