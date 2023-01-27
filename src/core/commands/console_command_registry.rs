use std::{collections::HashMap, sync::Arc};

use crate::core::commands::console_command::ConsoleCommand;

#[derive(Default)]
pub struct ConsoleCommandRegistry {
    commands: HashMap<String, Arc<dyn ConsoleCommand>>,
}

impl ConsoleCommandRegistry {
    pub fn new() -> Self {
        Self {
            commands: HashMap::<String, Arc<dyn ConsoleCommand>>::new(),
        }
    }

    pub fn add(&mut self, command: Arc<dyn ConsoleCommand>) -> &mut Self {
        self.commands.insert(command.get_name(), command);

        self
    }

    pub fn get(&self, command_name: &str) -> Option<&Arc<dyn ConsoleCommand>> {
        self.commands.get(command_name)
    }

    pub fn get_all_names(&self) -> Vec<String> {
        return self.commands.keys().cloned().collect();
    }
}
