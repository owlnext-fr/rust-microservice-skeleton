#[macro_export]
macro_rules! crash {
    ($message:expr) => {{
        use colored::Colorize;

        println!("");
        println!("{}", format!("{}", $message).red().bold());
        println!("");

        std::process::exit(99);
    }};
}
