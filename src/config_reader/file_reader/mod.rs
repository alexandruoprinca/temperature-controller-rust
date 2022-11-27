use std::io::BufRead;

use super::{ReadConfig, Config, extract_config_from_line};

pub struct ConfigFileReader {
    config_file_name: String,
}

impl ConfigFileReader {
    pub fn new(config_file_name: String) -> Self {
        ConfigFileReader { config_file_name }
    }
}


impl ReadConfig for ConfigFileReader {
    fn get_config(&self) -> Result<Option<Config>, Box<dyn std::error::Error>> {
        let mut buffer_reader =
            std::io::BufReader::new(std::fs::File::open(&self.config_file_name)?);
        let mut buffer = String::new();
        buffer_reader.read_line(&mut buffer)?;
        Ok(extract_config_from_line(&buffer))
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn correct_config() {
        let correct_config_file_path = "test_configs/correct_config.txt";
        let config_reader = ConfigFileReader {
            config_file_name: correct_config_file_path.to_string(),
        };
        let expected_min_temperature = -9.0;
        let expected_max_temperature = 15.0;

        let config = config_reader.get_config();
        assert!(config.is_ok());
        let config = config.unwrap();
        assert!(config.is_some());
        let config = config.unwrap();
        assert!(float_cmp::approx_eq!(
            f32,
            config.min_temperature,
            expected_min_temperature,
            epsilon = 0.000001
        ));
        assert!(float_cmp::approx_eq!(
            f32,
            config.max_temperature,
            expected_max_temperature,
            epsilon = 0.000001
        ));
    }

    #[test]
    fn missing_config_file() {
        let config_file_path = "test_configs/missing_file";
        let config_reader = ConfigFileReader {
            config_file_name: config_file_path.to_string(),
        };

        let config = config_reader.get_config();
        assert!(config.is_err());

        let config_error = config.err().unwrap();
        assert!(config_error.to_string() == "No such file or directory (os error 2)");
    }

    #[test]
    fn partial_config_file() {
        let config_file_path = "test_configs/partial_config.txt";
        let config_reader = ConfigFileReader {
            config_file_name: config_file_path.to_string(),
        };

        let config = config_reader.get_config();
        assert!(config.is_ok());

        let config = config.unwrap();
        assert!(config.is_none());
    }

    #[test]
    fn garbage_config_file() {
        let config_file_path = "test_configs/garbage_config.txt";
        let config_reader = ConfigFileReader {
            config_file_name: config_file_path.to_string(),
        };

        let config = config_reader.get_config();
        assert!(config.is_ok());

        let config = config.unwrap();
        assert!(config.is_none());
    }

    #[test]
    fn empty_config_file() {
        let config_file_path = "test_configs/empty_config.txt";
        let config_reader = ConfigFileReader {
            config_file_name: config_file_path.to_string(),
        };

        let config = config_reader.get_config();
        assert!(config.is_ok());

        let config = config.unwrap();
        assert!(config.is_none());
    }
}
