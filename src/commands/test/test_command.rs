use crate::{
    core::commands::{
        console_command::{CommandResult, ConsoleCommand},
        console_command_utils::ConsoleIO,
    },
    middlewares::cron_log_middleware::CronLogMiddleware,
};

use anyhow::Result;
use std::{
    collections::{BTreeMap, HashMap},
    thread, time, vec,
};

pub struct TestCommand {
    cron_log_middleware: CronLogMiddleware,
}

impl TestCommand {
    pub fn new(cron_log_middleware: CronLogMiddleware) -> Self {
        Self {
            cron_log_middleware,
        }
    }
}

#[async_trait]
impl ConsoleCommand for TestCommand {
    fn get_name(&self) -> String {
        "console:utils:test".into()
    }

    fn get_cron_middleware(&self) -> &CronLogMiddleware {
        &self.cron_log_middleware
    }

    async fn do_run(&self, _args: &HashMap<String, Option<String>>) -> Result<CommandResult> {
        let io = ConsoleIO::new();

        io.title("Hello there, i am title");

        io.new_line();

        io.section("ima section (d'assault, lol mdr)");
        io.new_line();

        io.write("print");
        io.writeln("  ---println in line end");
        io.writeln("println");

        io.new_line();

        io.listing(vec!["apples", "bananas", "oranges"]);

        io.new_line();

        io.comment("I am a comment");

        io.new_line();

        io.success("Yes ! it's a success !");

        io.new_line();

        io.error("Ew ! it's an error :(");

        io.new_line();

        io.warning("Beware ! it's a warning :o");

        io.new_line();

        io.info("Hmmm ! interesting info");

        io.new_line();

        io.note("Oh ! should note this note !");

        let headers = vec!["Country", "ISO-3166-2", "ISO-3166-3"];
        let data = vec![
            vec!["France", "FR", "FRA"],
            vec!["Espagne", "SP", "SPA"],
            vec!["Etats-Unis", "US", "USA"],
        ];

        io.new_line();

        io.table(headers, data);

        io.new_line();

        let mut map = BTreeMap::<String, String>::new();
        map.insert("Rust".into(), "Rust est un langage de programmation compilé multi-paradigme conçu et développé par Mozilla Research depuis 20106. Il a été conçu pour être « un langage fiable, concurrent, pratique »7,8, supportant les styles de programmation purement fonctionnel, modèle d'acteur, procédural, ainsi qu'orienté objet sous certains aspects9.".into());
        map.insert("PHP".into(), "PHP: Hypertext Preprocessor40, plus connu sous son sigle PHP (sigle auto-référentiel), est un langage de programmation libre41, principalement utilisé pour produire des pages Web dynamiques via un serveur HTTP40, mais pouvant également fonctionner comme n'importe quel langage interprété de façon locale. PHP est un langage impératif orienté objet.".into());
        map.insert("Python".into(), "Python (prononcé /pi.tɔ̃/) est un langage de programmation interprété, multiparadigme et multiplateformes. Il favorise la programmation impérative structurée, fonctionnelle et orientée objet. Il est doté d'un typage dynamique fort, d'une gestion automatique de la mémoire par ramasse-miettes et d'un système de gestion d'exceptions ; il est ainsi similaire à Perl, Ruby, Scheme, Smalltalk et Tcl.".into());

        io.definition_list(map);

        io.new_line();

        io.key_value_pair(vec![
            ("Application ID", "soiufghsqrpoiudshqfoih".into()),
            ("Login", "root".into()),
            ("Password", "root".into()),
        ]);

        io.new_line();

        let who = io.ask_question("Who dis:");
        io.writeln(&format!("Hello {who}"));

        io.new_line();

        let crab = io.ask_question_default("who best crab:", "Ferris");
        io.writeln(&format!("best crab is {crab}"));

        io.new_line();

        let confirmation = io.ask_confirm("Command good?");
        io.writeln(&format!("good ? {confirmation}"));

        io.new_line();
        let password = io.ask_password("Password ?");
        io.writeln(&format!("haha, got your password -> {password}"));

        io.new_line();

        let pb = io.create_progress_bar(100);
        let ten_millis = time::Duration::from_millis(10);

        for _ in 1..100 {
            thread::sleep(ten_millis);
            pb.inc(1);
        }

        pb.finish();

        io.new_line();
        io.new_line();

        let spinner = io.create_spinner();

        for _ in 1..500 {
            thread::sleep(ten_millis);
            spinner.tick();
        }

        spinner.finish();

        io.new_line();
        io.new_line();

        io.step(1, 4, "📝 learn rust...");
        io.step(2, 4, "🦀 raise crabs...");
        io.step(3, 4, "❔ ???");
        io.step(4, 4, "💲 profit !");

        Ok(CommandResult::SUCCESS)
    }
}
