use std::collections::BTreeMap;

use comfy_table::{
    modifiers::UTF8_ROUND_CORNERS,
    presets::{NOTHING, UTF8_FULL},
    Attribute, Cell, CellAlignment, Table,
};
use console::{style, Emoji, Term};
use indicatif::ProgressBar;
use inquire::{Confirm, DateSelect, MultiSelect, Password, PasswordDisplayMode, Select, Text};

const HEAVY_SEPARATOR: &str = "==================================";
const LIGHT_SEPARATOR: &str = "----------------------------------";
pub const LIST_SEPARATOR: &str = "$__SEPARATOR__$";
const LIST_REAL_SEPARATOR: &str = "----------";

pub enum InputType {
    Text,
    Date,
    Select,
    MultiSelect,
    Confirm,
    Password,
}

pub struct ConsoleIO {
    stdout: Term,
    stderr: Term,
}

#[allow(clippy::new_without_default)]
impl ConsoleIO {
    pub fn new() -> Self {
        Self {
            stdout: Term::stdout(),
            stderr: Term::stderr(),
        }
    }

    pub fn write(&self, text: &str) {
        self.stdout.write_str(text).unwrap();
    }

    pub fn writeln(&self, text: &str) {
        self.stdout.write_line(text).unwrap();
    }

    pub fn writeln_bold(&self, text: &str) {
        self.stdout
            .write_line(&format!("{}", style(text).white().bold()))
            .unwrap();
    }

    pub fn new_line(&self) {
        self.stdout.write_line("").unwrap();
    }

    pub fn title(&self, title: &str) {
        self.new_line();
        self.stdout
            .write_line(&format!("{}", style(title).yellow().bold()))
            .unwrap();
        self.stdout
            .write_line(&format!("{}", style(HEAVY_SEPARATOR).yellow().bold()))
            .unwrap();
    }

    pub fn section(&self, title: &str) {
        self.new_line();
        self.stdout
            .write_line(&format!("{}", style(title).cyan().bold()))
            .unwrap();
        self.stdout
            .write_line(&format!("{}", style(LIGHT_SEPARATOR).cyan().bold()))
            .unwrap();
    }

    pub fn comment(&self, comment: &str) {
        self.stdout
            .write_line(&format!("// {}", style(comment).white().dim().bold()))
            .unwrap();
    }

    pub fn step(&self, nb: usize, max: usize, message: &str) {
        let step_str = format!("[{nb}/{max}]");

        self.stdout
            .write_line(&format!(
                "{} {}",
                style(step_str).white().dim().bold(),
                style(message).white().bold()
            ))
            .unwrap();
    }

    pub fn success(&self, text: &str) {
        let success_symb_str = format!("[{} SUCCESS]", Emoji("✅", "✓"));

        self.stdout
            .write_line(&format!(
                "{} {}",
                style(success_symb_str).green().bold(),
                style(text).white().bold()
            ))
            .unwrap();
    }

    pub fn error(&self, text: &str) {
        let error_symb_str = format!("[{} ERROR]", Emoji("❌", "X"));

        self.stderr
            .write_line(&format!(
                "{} {}",
                style(error_symb_str).red().bold(),
                style(text).white().bold()
            ))
            .unwrap();
    }

    pub fn warning(&self, text: &str) {
        let warning_symb_str = format!("[{}  WARNING]", Emoji("⚠️", "!"));

        self.stdout
            .write_line(&format!(
                "{} {}",
                style(warning_symb_str).yellow().bold(),
                style(text).white().bold()
            ))
            .unwrap();
    }

    pub fn note(&self, text: &str) {
        let note_symb_str = format!("[{} NOTE]", Emoji("📘", "🕮"));

        self.stdout
            .write_line(&format!(
                "{} {}",
                style(note_symb_str).cyan().bold(),
                style(text).white().bold()
            ))
            .unwrap();
    }

    pub fn info(&self, text: &str) {
        let note_symb_str = format!("[{} INFO]", Emoji("📝", "▤"));

        self.stdout
            .write_line(&format!(
                "{} {}",
                style(note_symb_str).magenta().bold(),
                style(text).white().bold()
            ))
            .unwrap();
    }

    pub fn listing(&self, list: Vec<&str>) {
        list.iter()
            .map(|item| self.writeln(&format!("• {item}")))
            .for_each(drop);
    }

    pub fn table(&self, headers: Vec<&str>, data: Vec<Vec<&str>>) {
        let mut table = Table::new();

        let header_bold = headers
            .iter()
            .map(|e| {
                Cell::new(e)
                    .add_attribute(Attribute::Bold)
                    .set_alignment(CellAlignment::Center)
            })
            .collect::<Vec<Cell>>();

        table
            .load_preset(UTF8_FULL)
            .apply_modifier(UTF8_ROUND_CORNERS)
            .set_header(header_bold)
            .add_rows(data);

        self.writeln(&format!("{table}"));
    }

    pub fn key_value_pair(&self, values: Vec<(&str, String)>) {
        let mut table = Table::new();
        table.load_preset(NOTHING);

        for (key, value) in values.iter() {
            if value == LIST_SEPARATOR {
                table.add_row(vec![
                    Cell::new(LIST_REAL_SEPARATOR).add_attribute(Attribute::Bold),
                    Cell::new(""),
                ]);
            } else {
                table.add_row(vec![
                    Cell::new(key).add_attribute(Attribute::Bold),
                    Cell::new(value),
                ]);
            }
        }

        self.writeln(&format!("{table}"));
    }

    pub fn definition_list(&self, values: BTreeMap<String, String>) {
        let mut table = Table::new();
        table.load_preset(NOTHING);

        let mut i = 0;
        let len = values.len();

        for (key, value) in values.iter() {
            table.add_row(vec![
                Cell::new(key).add_attribute(Attribute::Bold),
                Cell::new(value),
            ]);

            i += 1;

            if i != len {
                table.add_row(vec!["", ""]);
            }
        }

        self.writeln(&format!("{table}"));
    }

    pub fn input_text<'a>(&'a self, question: &'a str) -> Text {
        Text::new(question)
    }

    pub fn input_date<'a>(&'a self, question: &'a str) -> DateSelect {
        DateSelect::new(question)
    }

    pub fn input_select<'a>(&'a self, question: &'a str, choices: Vec<&'a str>) -> Select<&'a str> {
        Select::new(question, choices)
    }

    pub fn input_multi_select<'a>(
        &'a self,
        question: &'a str,
        choices: Vec<&'a str>,
    ) -> MultiSelect<&'a str> {
        MultiSelect::new(question, choices)
    }

    pub fn input_confirm<'a>(&'a self, question: &'a str) -> Confirm {
        Confirm::new(question)
    }

    pub fn input_password<'a>(&'a self, question: &'a str) -> Password {
        Password::new(question)
    }

    pub fn ask_question(&self, question: &str) -> String {
        let mut response = self.input_text(question).prompt();

        while response.is_err() {
            self.error("An error occured while data input, please try again");

            response = Text::new(question).prompt();
        }

        response.unwrap()
    }

    pub fn ask_question_default(&self, question: &str, default: &str) -> String {
        let mut response = self.input_text(question).with_default(default).prompt();

        while response.is_err() {
            self.error("An error occured while data input, please try again");

            response = Text::new(question).prompt();
        }

        response.unwrap()
    }

    pub fn ask_confirm(&self, question: &str) -> bool {
        let mut response = self.input_confirm(question).with_default(true).prompt();

        while response.is_err() {
            self.error("An error occured while data input, please try again");

            response = self.input_confirm(question).prompt();
        }

        response.unwrap()
    }

    pub fn ask_password(&self, question: &str) -> String {
        let mut response = self
            .input_password(question)
            .with_display_mode(PasswordDisplayMode::Masked)
            .prompt();

        while response.is_err() {
            self.error("An error occured while data input, please try again");

            response = self
                .input_password(question)
                .with_display_mode(PasswordDisplayMode::Masked)
                .prompt();
        }

        response.unwrap()
    }

    pub fn create_progress_bar(&self, max: u64) -> ProgressBar {
        ProgressBar::new(max)
    }

    pub fn create_spinner(&self) -> ProgressBar {
        ProgressBar::new_spinner()
    }
}
