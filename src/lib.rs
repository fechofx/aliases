extern crate yaml_rust;
extern crate rustache;
extern crate crypto;

mod aliases;

use aliases::commands::init::Init;
use aliases::commands::list::List;
pub use aliases::commands::rehash::Rehash;
pub use aliases::builders::AliasBuilder; // had to do this for the tests, why?
pub use aliases::models::Alias; // had to do this for the tests, why?
pub use aliases::factories::AliasFactory; // had to do this for the tests, why?
pub use aliases::collections::Aliases; // had to do this for the tests, why?
pub use aliases::factories::ShimFileFactory; // had to do this for the tests, why?

use std::env;
use std::path::{Path, PathBuf};
use yaml_rust::{Yaml, YamlLoader};
use std::io::prelude::*;
use std::fs::File;

struct TemplateRepository;

impl  TemplateRepository {

    pub fn config_template() -> String {
        "shim_directory: $HOME/.aliases.d/shims\naliases_directories: []".to_string() // TODO what willl this actually be?
    }

}

struct Config {
    shim_directory: PathBuf,
    alias_directories: Vec<PathBuf>,
}

impl Config {

    pub fn load() -> Config {
        let config;
        if Config::config_file_path().exists() {
            config = Config::load_file(&Config::config_file_path());
        } else {
            config = Config::create(&Config::config_file_path());
        }
        let aliases_directories = config["aliases_directories"].as_vec().unwrap().into_iter().map(|dir| PathBuf::from(dir.as_str().unwrap())).collect();
        Config {
            shim_directory: PathBuf::from(config["shim_directory"].as_str().unwrap()),
            alias_directories: aliases_directories,
        }
    }

    // ------- private methods --------//

    fn config_file_path() -> PathBuf {
        match env::var("HOME") {
            Ok(home_dir) => {
                PathBuf::from(home_dir)
            },
            Err(_) => {
                PathBuf::new() // need to handle this better
            },
        }
    }

    fn create(path: &PathBuf) -> Yaml {
        let mut file = File::create(path).unwrap(); //TODO handle the error case
        let default_config = TemplateRepository::config_template();
        file.write_all(default_config.as_bytes()).unwrap();
        Yaml::from_str(&default_config)
    }

    fn load_file(path: &PathBuf) -> Yaml {
        let mut file = File::open(path).unwrap(); //TODO handle the error case
        let mut content = String::new();
        file.read_to_string(&mut content);
        YamlLoader::load_from_str(&content).unwrap()[0].clone()
    }
}


pub struct App {
    config: Config,
    pub current_path: PathBuf,
}

impl App {

    pub fn new() -> App {
        let config = Config::load();
        App {
            config: config,
            current_path: env::current_dir().unwrap(),
        }
    }

    pub fn execute_init(&mut self, global: bool) {
        let target_path;
        if global {
            match env::var("HOME") {
                Ok(home_dir) => {
                   target_path = PathBuf::from(home_dir);
                },
                Err(_) => {
                    target_path = self.current_path.clone();
                },
            }
        } else {
            target_path = self.current_path.clone();
        }
        Init::new(target_path).execute();
    }

    pub fn execute_list(&mut self) {
        // TODO this needs to be able to filter by directory and return exit codes
        List::new(self.current_path.clone()).execute();
    }

    pub fn execute_rehash(&mut self) {
        Rehash::new(self.config.shim_directory.clone(), self.config.alias_directories.clone()).execute();
    }
}
