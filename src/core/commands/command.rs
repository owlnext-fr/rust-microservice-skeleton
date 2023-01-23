use std::collections::HashMap;

pub struct CommandHandle<'a, T: ?Sized + Send + Sync> {
    command: Box<T>,
    schedule: &'a str,
    args: HashMap<&'a str, &'a str>,
}

impl<'a, T> CommandHandle<'a, T> where T: Command + ?Sized + Send + Sync {}

pub trait Command {}
