#[macro_export]
macro_rules! command_error {
    ($error_message:expr) => {{
        let io = ConsoleIO::new();

        io.error($error_message);

        return Ok(CommandResult::ERROR($error_message.to_owned()));
    }};
}

#[macro_export]
macro_rules! command_success {
    () => {{
        return Ok(CommandResult::SUCCESS);
    }};
}
