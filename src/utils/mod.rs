use std::env;
mod config;

#[derive(Clone)]
pub struct Utils {
    base_folder: String,
    abspath: String,
}

impl Utils {
    pub fn new() -> Utils {
        Utils {
            base_folder: String::from("src"),
            abspath: String::from(env::current_dir().unwrap().to_str().unwrap()),
        }
    }

    /// # Get current lang
    // update code to check for other locales
    fn get_current_lang_path(&self, s: &str) -> String {
        //@TODO more logic here to get the template for the current language
        format!("{}/{}", s, "default/")
    }

    /// # Get base folder
    fn get_base_folder(&self) -> String {
        self.base_folder.clone()
    }

    /// # Get absolute path
    pub fn get_abs_path(&self) -> String {
        self.abspath.clone()
    }

    /// # Get admin path
    pub fn get_admin_path(&self) -> String {
        let config_object = self.get_config("config");
        let admin_folder = config_object.get("paths.admin_folder");
        if admin_folder.is_none() {
            panic!("no 'admin_folder' configuration exist!");
        }
        let templates_folder = config_object.get("paths.templates_folder");
        if templates_folder.is_none() {
            panic!("no 'templates_folder' configuration exist!");
        }
        let admin_path: String = format!("{}/{}/{}/{}", self.get_abs_path(), self.get_base_folder(), admin_folder.unwrap().as_str().unwrap(), templates_folder.unwrap().as_str().unwrap());
        self.get_current_lang_path(&admin_path[..])
    }

    /// # Get content path
    pub fn get_content_path(&self) -> String {
        let config_object = self.get_config("config");
        let content_folder = config_object.get("paths.content_folder");
        if content_folder.is_none() {
            panic!("no 'admin_folder' configuration exist!");
        }
        let themes_folder = config_object.get("paths.content_themes");
        if themes_folder.is_none() {
            panic!("no 'themes_folder' configuraion exist!");
        }
        let active_theme = config_object.get("paths.active_theme");
        if active_theme.is_none() {
            panic!("no 'active_theme' configuraion exist!");
        }
        let templates_folder = config_object.get("paths.templates_folder");
        if templates_folder.is_none() {
            panic!("no 'templates_folder' configuration exist!");
        }
        let content_path: String = format!("{}/{}/{}/{}/{}/{}", self.get_abs_path(), self.get_base_folder(), content_folder.unwrap().as_str().unwrap(), themes_folder.unwrap().as_str().unwrap(), active_theme.unwrap().as_str().unwrap(), templates_folder.unwrap().as_str().unwrap());
        self.get_current_lang_path(&content_path[..])
    }

    /// # Load configuration file
    fn load_file(&self, file_path: &str) -> String {
        use std::fs::File;
        use std::io::prelude::*;
        use std::io::Read;

        let mut f = File::open(file_path).expect("Unable to open");
        let mut contents = String::new();
        f.read_to_string(&mut contents).expect("Error reading file");

        contents
    }

    /// # Parse toml file
    pub fn get_config(&self, str: &str) -> config::Config {
        let file_path = format!("{}/{}/{}.toml", &self.get_abs_path()[..], &self.base_folder[..], str);
        config::Config::new(&self.load_file(&file_path[..])[..])
    }
}
