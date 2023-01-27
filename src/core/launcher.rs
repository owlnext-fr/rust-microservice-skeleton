use std::collections::HashMap;

use anyhow::{bail, Result};
use clap::{builder::ValueParser, Parser, Subcommand};
use rocket::{Build, Rocket};

use super::commands::console_command_registry::ConsoleCommandRegistry;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct CLI {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand)]
pub enum Command {
    Launch,
    Console {
        sub_command: String,
        #[arg(num_args(0..), value_parser = ValueParser::new(parse_subcommand_args))]
        args: HashMap<String, Option<String>>,
    },
}

#[allow(unused_must_use)]
pub async fn engage(rocket: Rocket<Build>) -> Result<()> {
    rocket.ignite().await?.launch().await?;
    Ok(())
}

pub async fn warp(
    rocket: Rocket<Build>,
    sub_command: &str,
    args: &HashMap<String, Option<String>>,
) -> Result<()> {
    let command_registry = rocket.state::<ConsoleCommandRegistry>();

    if command_registry.is_none() {
        bail!("Cannot find command registry in rocket, aborting...");
    }

    let command_registry = command_registry.unwrap();

    let command = command_registry.get(sub_command);

    if command.is_none() {
        bail!(
            "Cannot find {sub_command} in command registry, found {:#?}",
            command_registry.get_all_names()
        );
    }

    let command = command.unwrap();

    command.run(args).await?;

    Ok(())
}

fn parse_subcommand_args(arg_str: &str) -> Result<HashMap<String, Option<String>>> {
    let mut args = HashMap::<String, Option<String>>::new();

    let arg_packs = arg_str.split(' ').collect::<Vec<&str>>();

    for arg_pack in arg_packs.iter() {
        if arg_pack.contains('=') {
            let arg_body = arg_pack.split('=').collect::<Vec<&str>>();

            let arg_name = arg_body.first().unwrap().to_string();
            let arg_value = arg_body.get(1).unwrap().to_string();

            args.insert(arg_name, Some(arg_value));
        } else {
            args.insert(arg_pack.to_string(), None);
        }
    }

    Ok(args)
}
