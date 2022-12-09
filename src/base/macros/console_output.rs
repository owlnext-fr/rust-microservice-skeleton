#[macro_export]
macro_rules! console_error {
    ($message:expr) => {{
        use colored::Colorize;

        println!("");
        println!("[ERROR]: {}", format!("{}", $message).red().bold());
        println!("");
    }};
}

#[macro_export]
macro_rules! console_warning {
    ($message:expr) => {{
        use colored::Colorize;

        println!("");
        println!("[ERROR]: {}", format!("{}", $message).yellow().bold());
        println!("");
    }};
}
