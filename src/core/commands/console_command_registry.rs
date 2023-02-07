use std::{collections::HashMap, sync::Arc};

use crate::core::commands::console_command::ConsoleCommand;

/// A struct to centralize ConsoleCommand registration into the application.
#[derive(Default)]
pub struct ConsoleCommandRegistry {
    /// the actual registry of ConsoleCommand
    commands: HashMap<String, Arc<dyn ConsoleCommand>>,
}

impl ConsoleCommandRegistry {
    /// creates a new instance.
    pub fn new() -> Self {
        Self {
            commands: HashMap::<String, Arc<dyn ConsoleCommand>>::new(),
        }
    }

    /// adds a ConsoleCommand to the registry, using it's name as a key.
    ///
    /// **Note:** The command instance must be Arc'ed.
    pub fn add(&mut self, command: Arc<dyn ConsoleCommand>) -> &mut Self {
        self.commands.insert(command.get_name(), command);

        self
    }

    /// gets a given command by it's unique name.
    pub fn get(&self, command_name: &str) -> Option<&Arc<dyn ConsoleCommand>> {
        self.commands.get(command_name)
    }

    /// gets names of all the commands present in the current registry.
    pub fn get_all_names(&self) -> Vec<String> {
        return self.commands.keys().cloned().collect();
    }
}
