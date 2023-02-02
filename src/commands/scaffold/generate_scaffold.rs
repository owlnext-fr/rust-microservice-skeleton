use std::{collections::HashMap, fs, path::PathBuf};

use crate::{
    command_error, command_success,
    core::commands::{
        console_command::{CommandResult, ConsoleCommand},
        console_command_utils::ConsoleIO,
    },
    middlewares::cron_log_middleware::CronLogMiddleware,
};
use inflections::Inflect;

use anyhow::Result;

#[derive(Debug, Clone)]
struct ScaffoldNames {
    pub module_name: String,
    pub module_name_plural: String,
    pub controller_name: String,
    pub dto_module_name: String,
    pub data_class_struct_name: String,
    pub new_data_class_struct_name: String,
    pub data_repository_struct_name: String,
    pub middleware_struct_name: String,
    pub security_voter_struct_name: String,
}

impl From<&str> for ScaffoldNames {
    fn from(value: &str) -> Self {
        Self {
            module_name: value.to_lowercase(),
            module_name_plural: format!("{}s", value.to_lowercase()),
            controller_name: value.to_lowercase(),
            dto_module_name: value.to_lowercase(),
            data_class_struct_name: value.to_pascal_case(),
            new_data_class_struct_name: format!("New{}", value.to_pascal_case()),
            data_repository_struct_name: format!("{}Repository", value.to_pascal_case()),
            middleware_struct_name: format!("{}Middleware", value.to_pascal_case()),
            security_voter_struct_name: format!("{}SecurityVoter", value.to_pascal_case()),
        }
    }
}

pub struct GenerateScaffold {
    cron_log_middleware: CronLogMiddleware,
}

impl GenerateScaffold {
    pub fn new(cron_log_middleware: CronLogMiddleware) -> Self {
        Self {
            cron_log_middleware,
        }
    }
}

#[async_trait]
impl ConsoleCommand for GenerateScaffold {
    fn get_name(&self) -> String {
        "make:generate-scaffold".into()
    }

    fn get_cron_middleware(&self) -> &CronLogMiddleware {
        &self.cron_log_middleware
    }

    async fn do_run(&self, args: &HashMap<String, Option<String>>) -> Result<CommandResult> {
        let io = ConsoleIO::new();

        io.title("Scafforld generation");
        io.comment("This helper will generate a code scaffold to ease API development.");
        io.new_line();

        let struct_name = if args.contains_key("struct-name") {
            args.get("struct-name").unwrap().clone().unwrap()
        } else {
            io.ask_question("Enter struct name:")
        };

        let scaffold_names = &ScaffoldNames::from(struct_name.as_str());

        io.new_line();
        io.warning(&format!(
            "You are about to generate a code scaffold for struct {}",
            scaffold_names.module_name
        ));

        io.key_value_pair(vec![
            ("module name", scaffold_names.module_name.clone()),
            ("Controller name", scaffold_names.controller_name.clone()),
            ("DTO module", scaffold_names.dto_module_name.clone()),
            (
                "Data class struct",
                scaffold_names.data_class_struct_name.clone(),
            ),
            (
                "New data class struct",
                scaffold_names.new_data_class_struct_name.clone(),
            ),
            (
                "Data repository struct",
                scaffold_names.data_repository_struct_name.clone(),
            ),
            (
                "Middleware struct",
                scaffold_names.middleware_struct_name.clone(),
            ),
            (
                "Security voter struct",
                scaffold_names.security_voter_struct_name.clone(),
            ),
        ]);

        io.new_line();
        let confirm = io.ask_confirm("Are you sure ?");

        if !confirm {
            command_error!("Aborted !");
        }
        /* #region preparation */

        io.new_line();
        io.step(1, 8, "Preparing content...");
        let names = &scaffold_names.clone();
        let template_dir = &PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("src")
            .join("commands")
            .join("scaffold")
            .join("templates");

        let validation_file = &PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("src")
            .join("domain")
            .join("model")
            .join(format!("{}.rs", names.module_name));

        if validation_file.exists() {
            command_error!(&format!("{} module already exists !", names.module_name));
        }

        /* #endregion */

        /* #region data class */
        io.step(2, 8, "Creating data struct...");

        let template_file = &template_dir.join("_data_struct.rs");

        let folder = &PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("src")
            .join("domain")
            .join("model");

        let file = &folder.join(format!("{}.rs", names.module_name));

        let mod_file = &folder.join("mod.rs");

        let content = fs::read_to_string(template_file);

        if content.is_err() {
            command_error!(&format!(
                "Cannot read data class template file: {}",
                content.as_ref().err().unwrap()
            ));
        }

        let content = content
            .unwrap()
            .replace("__MODULE_NAME_PLURAL__", &names.module_name_plural)
            .replace("__DATA_CLASS_STRUCT_NAME__", &names.data_class_struct_name)
            .replace(
                "__NEW_DATA_CLASS_STRUCT_NAME__",
                &names.new_data_class_struct_name,
            );

        let written = fs::write(file, content);

        if written.is_err() {
            command_error!(&format!(
                "Cannot write data class template file: {}",
                written.as_ref().err().unwrap()
            ));
        }

        let mod_content = fs::read_to_string(mod_file);

        if mod_content.is_err() {
            command_error!(&format!(
                "Cannot read data class mod file: {}",
                mod_content.as_ref().err().unwrap()
            ));
        }

        let mut mod_content = mod_content.unwrap();
        mod_content.push_str(&format!("pub mod {};\n", names.module_name));

        let written = fs::write(mod_file, mod_content);

        if written.is_err() {
            command_error!(&format!(
                "Cannot write data class into data class mod file: {}",
                written.as_ref().err().unwrap()
            ));
        }
        /* #endregion */

        /* #region DTO */
        io.step(3, 8, "Creating DTOs...");

        let template_file = &template_dir.join("_dto.rs");

        let folder = &PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("src")
            .join("domain")
            .join("dto");

        let file = &folder.join(format!("{}.rs", names.module_name));

        let mod_file = &folder.join("mod.rs");

        let content = fs::read_to_string(template_file);

        if content.is_err() {
            command_error!(&format!(
                "Cannot read DTO file: {}",
                content.as_ref().err().unwrap()
            ));
        }

        let content = content
            .unwrap()
            .replace("__MODULE_NAME__", &names.module_name)
            .replace("__DATA_CLASS_STRUCT_NAME__", &names.data_class_struct_name);

        let written = fs::write(file, content);

        if written.is_err() {
            command_error!(&format!(
                "Cannot write DTO file: {}",
                written.as_ref().err().unwrap()
            ));
        }

        let mod_content = fs::read_to_string(mod_file);

        if mod_content.is_err() {
            command_error!(&format!(
                "Cannot DTO mod file: {}",
                mod_content.as_ref().err().unwrap()
            ));
        }

        let mut mod_content = mod_content.unwrap();
        mod_content.push_str(&format!("pub mod {};\n", names.module_name));

        let written = fs::write(mod_file, mod_content);

        if written.is_err() {
            command_error!(&format!(
                "Cannot write DTO into DTO mod file: {}",
                written.as_ref().err().unwrap()
            ));
        }
        /* #endregion */

        /* #region Repository */
        io.step(4, 8, "Creating data repository...");

        let mod_name = format!("{}_repository", names.module_name);

        let template_file = &template_dir.join("_data_repository.rs");

        let folder = &PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("src")
            .join("domain")
            .join("repository");

        let file = &folder.join(format!("{mod_name}.rs"));

        let mod_file = &folder.join("mod.rs");

        let content = fs::read_to_string(template_file);

        if content.is_err() {
            command_error!(&format!(
                "Cannot read repository template file: {}",
                content.as_ref().err().unwrap()
            ));
        }

        let content = content
            .unwrap()
            .replace("__MODULE_NAME__", &names.module_name)
            .replace("__MODULE_NAME_PLURAL__", &names.module_name_plural)
            .replace("__DATA_CLASS_STRUCT_NAME__", &names.data_class_struct_name)
            .replace(
                "__NEW_DATA_CLASS_STRUCT_NAME__",
                &names.new_data_class_struct_name,
            );

        let written = fs::write(file, content);

        if written.is_err() {
            command_error!(&format!(
                "Cannot write repository file: {}",
                written.as_ref().err().unwrap()
            ));
        }

        let mod_content = fs::read_to_string(mod_file);

        if mod_content.is_err() {
            command_error!(&format!(
                "Cannot read repository mod file: {}",
                mod_content.as_ref().err().unwrap()
            ));
        }

        let mut mod_content = mod_content.unwrap();
        mod_content.push_str(&format!("pub mod {mod_name};\n"));

        let written = fs::write(mod_file, mod_content);

        if written.is_err() {
            command_error!(&format!(
                "Cannot write repository into repository mod file: {}",
                written.as_ref().err().unwrap()
            ));
        }
        /* #endregion */

        /* #region Middleware */
        io.step(5, 8, "Creating middleware...");

        let mod_name = format!("{}_middleware", names.module_name);

        let template_file = &template_dir.join("_middleware.rs");

        let folder = &PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("src")
            .join("middlewares");

        let file = &folder.join(format!("{mod_name}.rs"));

        let mod_file = &folder.join("mod.rs");

        let content = fs::read_to_string(template_file);

        if content.is_err() {
            command_error!(&format!(
                "Cannot read middleware template file: {}",
                content.as_ref().err().unwrap()
            ));
        }

        let content = content
            .unwrap()
            .replace("__MODULE_NAME__", &names.module_name)
            .replace("__MODULE_NAME_PLURAL__", &names.module_name_plural)
            .replace("__DATA_CLASS_STRUCT_NAME__", &names.data_class_struct_name)
            .replace(
                "__NEW_DATA_CLASS_STRUCT_NAME__",
                &names.new_data_class_struct_name,
            );

        let written = fs::write(file, content);

        if written.is_err() {
            command_error!(&format!(
                "Cannot write middleware file: {}",
                written.as_ref().err().unwrap()
            ));
        }

        let mod_content = fs::read_to_string(mod_file);

        if mod_content.is_err() {
            command_error!(&format!(
                "Cannot read middleware mod file: {}",
                mod_content.as_ref().err().unwrap()
            ));
        }

        let mut mod_content = mod_content.unwrap();
        mod_content.push_str(&format!("pub mod {mod_name};\n"));

        let written = fs::write(mod_file, mod_content);

        if written.is_err() {
            command_error!(&format!(
                "Cannot write middleware into middleware mod file: {}",
                written.as_ref().err().unwrap()
            ));
        }
        /* #endregion */

        /* #region Security voter */
        io.step(6, 8, "Creating security voter...");

        let mod_name = format!("{}_security", names.module_name);

        let template_file = &template_dir.join("_security.rs");

        let folder = &PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("src")
            .join("security")
            .join("voters");

        let file = &folder.join(format!("{mod_name}.rs"));

        let mod_file = &folder.join("mod.rs");

        let content = fs::read_to_string(template_file);

        if content.is_err() {
            command_error!(&format!(
                "Cannot read security voter template file: {}",
                content.as_ref().err().unwrap()
            ));
        }

        let content = content
            .unwrap()
            .replace("__MODULE_NAME__", &names.module_name)
            .replace("__MODULE_NAME_PLURAL__", &names.module_name_plural)
            .replace("__DATA_CLASS_STRUCT_NAME__", &names.data_class_struct_name)
            .replace(
                "__NEW_DATA_CLASS_STRUCT_NAME__",
                &names.new_data_class_struct_name,
            );

        let written = fs::write(file, content);

        if written.is_err() {
            command_error!(&format!(
                "Cannot write security voter file: {}",
                written.as_ref().err().unwrap()
            ));
        }

        let mod_content = fs::read_to_string(mod_file);

        if mod_content.is_err() {
            command_error!(&format!(
                "Cannot read security voter mod file: {}",
                mod_content.as_ref().err().unwrap()
            ));
        }

        let mut mod_content = mod_content.unwrap();
        mod_content.push_str(&format!("pub mod {mod_name};\n"));

        let written = fs::write(mod_file, mod_content);

        if written.is_err() {
            command_error!(&format!(
                "Cannot write security voter into security voter mod file: {}",
                written.as_ref().err().unwrap()
            ));
        }
        /* #endregion */

        /* #region Controller */

        io.step(7, 8, "Creating controller...");

        let mod_name = names.module_name.to_string();

        let template_file = &template_dir.join("_controller.rs");

        let folder = &PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("src")
            .join("controllers")
            .join("api");

        let file = &folder.join(format!("{mod_name}.rs"));

        let mod_file = &folder.join("mod.rs");

        let content = fs::read_to_string(template_file);

        if content.is_err() {
            command_error!(&format!(
                "Cannot read controller template file: {}",
                content.as_ref().err().unwrap()
            ));
        }

        let content = content
            .unwrap()
            .replace("__MODULE_NAME__", &names.module_name)
            .replace("__MODULE_NAME_PLURAL__", &names.module_name_plural)
            .replace("__DATA_CLASS_STRUCT_NAME__", &names.data_class_struct_name)
            .replace(
                "__NEW_DATA_CLASS_STRUCT_NAME__",
                &names.new_data_class_struct_name,
            );

        let written = fs::write(file, content);

        if written.is_err() {
            command_error!(&format!(
                "Cannot write controller file: {}",
                written.as_ref().err().unwrap()
            ));
        }

        let mod_content = fs::read_to_string(mod_file);

        if mod_content.is_err() {
            command_error!(&format!(
                "Cannot read controller mod file: {}",
                mod_content.as_ref().err().unwrap()
            ));
        }

        let mut mod_content = mod_content.unwrap();
        mod_content.push_str(&format!("pub mod {mod_name};\n"));

        let written = fs::write(mod_file, mod_content);

        if written.is_err() {
            command_error!(&format!(
                "Cannot write controller into controller mod file: {}",
                written.as_ref().err().unwrap()
            ));
        }
        /* #endregion */

        /* #region Rocket builder */
        io.step(8, 8, "Adding modules into rocket builder...");

        let rocket_builder_file = &PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("src")
            .join("core")
            .join("rocket_factory.rs");

        let factory_content = fs::read_to_string(rocket_builder_file);

        if factory_content.is_err() {
            command_error!("Cannot open rocket factory file !");
        }

        let factory_content = factory_content.unwrap();

        let module_name = &names.module_name;
        let module_name_ucfirst = &names.data_class_struct_name;

        // -- imports --
        let import_insert = format!(
            "use crate::controllers::api::{module_name};
use crate::domain::repository::{module_name}_repository::{module_name_ucfirst}Repository;
use crate::middlewares::{module_name}_middleware::{module_name_ucfirst}Middleware;
use crate::security::voters::{module_name}_security::{module_name_ucfirst}SecurityVoter;
// __IMPORTS__"
        );

        let factory_content = factory_content.replace("// __IMPORTS__", &import_insert);
        // --

        // -- rep --
        let repository_insert = format!(
            "   let {module_name}_rep = {module_name_ucfirst}Repository::new(db_state.clone());
    // __REPOSITORY__"
        );
        let factory_content = factory_content.replace("    // __REPOSITORY__", &repository_insert);
        // --

        // -- middleware --
        let middleware_insert = format!(
            "   let {module_name}_middleware = {module_name_ucfirst}Middleware::new({module_name}_rep.clone());
    // __MIDDLEWARE__"
        );
        let factory_content = factory_content.replace("    // __MIDDLEWARE__", &middleware_insert);
        // --

        // -- security --
        let security_insert = format!(
            "   security.add_voter(Box::<{module_name_ucfirst}SecurityVoter>::default());
    // __SECURITY__"
        );
        let factory_content = factory_content.replace("    // __SECURITY__", &security_insert);
        // --

        // -- controllers --
        let controllers_insert = format!(
            "               {module_name}::{module_name}_list,
                {module_name}::{module_name}_details,
                {module_name}::{module_name}_create,
                {module_name}::{module_name}_update,
                {module_name}::{module_name}_delete,
                // __CONTROLLERS__"
        );
        let factory_content =
            factory_content.replace("                // __CONTROLLERS__", &controllers_insert);
        // --

        // -- manage --
        let manage_insert = format!(
            ".manage({module_name}_middleware)
            // __MANAGE__"
        );
        let factory_content = factory_content.replace("        // __MANAGE__", &manage_insert);
        // --

        let written = fs::write(rocket_builder_file, factory_content);

        if written.is_err() {
            command_error!("Cannot write rocket builder file !");
        }

        /* #endregion */

        io.new_line();
        io.success("Scaffold successfully generated !");

        io.new_line();
        io.writeln("You should:");
        io.listing(vec![
            "Create and run your migration if not done already (don't forget to patch schema.rs).",
            "Fill all the DTOs and middlewares",
            "Complete repositories if needed",
            "Write your business logic",
        ]);

        io.new_line();
        io.writeln_bold("Let's launch this rocket 🦀🚀");
        io.new_line();

        command_success!();
    }
}
