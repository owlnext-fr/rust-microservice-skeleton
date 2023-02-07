use std::collections::HashMap;

use anyhow::{bail, Result};
use clap::{builder::ValueParser, Parser, Subcommand};
use rocket::{Build, Rocket};

use super::commands::console_command_registry::ConsoleCommandRegistry;

/// A structure representing the principal running option for this program.
///
/// As this program takes a mandatory first argument, the real functional description is in the Command struct.
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct CLI {
    /// sub-command to actually run a part of the program.
    #[command(subcommand)]
    pub command: Command,
}

/// actual mode switching intelligence for the program.
///
/// The `launch` argument will launch the web API as a long-running program.
/// The `console` argument will launch the command interface, executing a one-time command.
#[derive(Subcommand)]
pub enum Command {
    /// launches the web API.
    Launch,
    /// launches the command interface.
    Console {
        /// the current command name to launch
        sub_command: String,
        /// args for this command, with format "<key>=<value>;<flag>;..."
        #[arg(num_args(0..), value_parser = ValueParser::new(parse_subcommand_args))]
        args: HashMap<String, Option<String>>,
    },
}

/// launches the web API.
/// This will ignite the Rocket and start the rocket engine.
#[allow(unused_must_use)]
pub async fn engage(rocket: Rocket<Build>) -> Result<()> {
    rocket.ignite().await?.launch().await?;
    Ok(())
}

/// launches the command interface.
/// This will invoke the builtin ConsoleCommandRegistry from rocket state registry, then parse arguments
/// and finally launch the command stored in the ConsoleCommandRegistry.
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

/// This function will parse the arg string into a map formatted as KEY => Option(VALUE).
///
/// The format of the args must be :
/// - `key=val` for key-value pairs
/// - `flag` for flags only
///
/// All separated by `;`
fn parse_subcommand_args(arg_str: &str) -> Result<HashMap<String, Option<String>>> {
    let mut args = HashMap::<String, Option<String>>::new();

    let arg_packs = arg_str.split(';').collect::<Vec<&str>>();

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
