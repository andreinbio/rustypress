use toml;

pub struct Config {
    value: Option<toml::Value>,
}

impl Config {
    pub fn new(str: &str) -> Config {
        Config {
            value: str.parse::<toml::Value>().ok()
        }
    }

    /// # Get the value of the preference
    pub fn get(&self, str: &str) -> Option<&toml::Value> {
        let strings: Vec<&str> = str.split(".").collect::<Vec<&str>>();
        let mut config_value: Option<&toml::Value> = self.value.as_ref();

        for item in &strings {
            if config_value.is_some() {
                match config_value.unwrap().get(item) {
                    Some(value) => {
                        config_value = Some(value);
                        match *value {
                            toml::Value::Array(_) | toml::Value::Table(_) => {
                                config_value = Some(value);
                            },
                            _ => {
                                break;
                            },
                        }
                    },
                    None => {
                        config_value = None;
                        break;
                    },
                }
            }
        }

        config_value
    }

    /// # Checks if the value of the preference is boolean true
    pub fn is(&self, str: &str) -> Option<bool> {
        let value: Option<&toml::Value> = self.get(str);

        if value.is_some() {
            return value.unwrap().as_bool()
        } else {
            return None
        }
    }
}