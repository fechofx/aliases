use std::path::{Path, PathBuf};
use std::io::prelude::*;
use std::fs::File;

use aliases::Config;
use aliases::commands::AliasCommand;
use aliases::commands::CommandResponse;

pub struct Init<'a> {
    target_path: PathBuf,
    config: Config,
    global: bool,
    user: Option<&'a str>,
}

impl<'a> Init<'a> {

    pub fn new(target_path: PathBuf, config: Config, global: bool, user: Option<&str>) -> Init {
        Init { target_path: target_path, config: config, global: global, user: user}
    }


    // ------------ private ---------- //

    fn create_aliases_file(&self) {
        let aliases_filename;
        match self.user {
            Some(user) => { aliases_filename = format!(".aliases-{}", user) },
            None => { aliases_filename = String::from(".aliases") },
        }
        if !Path::new(&self.target_path.join(&aliases_filename)).exists() {
            let mut new_file = File::create(self.target_path.join(&aliases_filename)).unwrap();
            let template_string = self.template_string();
            let array = template_string.as_bytes();
            let _ = new_file.write_all(array);
        }
    }

    fn template_string(&self) -> String {
String::from("# This file is autogenerated by the aliases tool.
# For more info about aliases type `aliases --help`
# or visit https://github.com/sebglazebrook/aliases

#alias_name:
#  command: ./super_command.sh                         # required
#  confirm: true                                       # optional
#  confirmation_message: Are you sure you are sure??   # optional
#  conditional: /bin/true                              # optional
#  backout_seconds: 3                                  # optional
#  unit_test: '[ true = true ]'                        # optional
#  quiet: false                                        # optional
")
    }

    fn add_to_global_config(&self) {
        Config::load().add_alias_directory(&self.target_path, &self.user);
    }
}

impl<'a> AliasCommand for Init<'a> {

    fn execute(&self) -> CommandResponse {
        if self.global {
            let path_update = String::from("export PATH=\"") + &self.config.shim_directory +  ":${PATH}\"";
            // what the hell was I trying to do here?
            println!("{}\naliases rehash", path_update);
        } else {
            self.create_aliases_file();
            self.add_to_global_config();
        }
        CommandResponse::success()
    }
}
