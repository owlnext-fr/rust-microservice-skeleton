#[macro_export]
macro_rules! extract_message {
    ($error:expr) => {{
        format!("{}", $error.err().unwrap().root_cause())
    }};
}
