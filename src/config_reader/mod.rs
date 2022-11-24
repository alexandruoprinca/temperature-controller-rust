use std::io::BufRead;

pub struct Config {
    pub min_temperature: f32,
    pub max_temperature: f32,
}

#[mockall::automock]
pub trait ParseConfig {
    fn get_config(&self) -> Result<Option<Config>, std::io::Error>;
}

pub struct ConfigReader {
    config_file_name: String
}

impl ConfigReader {
    pub fn new(config_file_name: String) -> Self {
        ConfigReader { config_file_name }
    }
}

impl ParseConfig for ConfigReader {
    fn get_config(&self) -> Result<Option<Config>, std::io::Error> {
        let mut buffer_reader = std::io::BufReader::new(std::fs::File::open(&self.config_file_name)?);
        let mut buffer = String::new();

        buffer_reader.read_line(&mut buffer)?;
        let values = buffer.split_whitespace().collect::<Vec<&str>>();
        if values.len() < 2 {
            eprintln!("Did not enter correct config values");
            return Ok(None);
        }

        let min_temperature = match values[0].trim().parse::<f32>() {
            Ok(result) => result,
            Err(err) => {
                eprintln!("Receiving data {buffer}");
                eprintln!("Failed to convert min temp to a number. Received min temp is {err}");
                return Ok(None);
            }
        };

        let max_temperature = match values[1].trim().parse::<f32>() {
            Ok(result) => result,
            Err(err) => {
                eprintln!("Receiving data {buffer}");
                eprintln!("Failed to convert max temp to a number. Received max temp is {err}");
                return Ok(None);
            }
        };

        Ok(Some(Config {
            min_temperature,
            max_temperature,
        }))
    }
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn correct_config() {
        let correct_config_file_path = "test_configs/correct_config.txt";
        let config_reader = ConfigReader{
            config_file_name:correct_config_file_path.to_string()
        };
        let expected_min_temperature = -9.0;
        let expected_max_temperature = 15.0;

        let config = config_reader.get_config();
        assert!(config.is_ok());
        let config = config.unwrap();
        assert!(config.is_some());
        let config = config.unwrap();
        assert!(float_cmp::approx_eq!(f32, config.min_temperature, expected_min_temperature, epsilon=0.000001));
        assert!(float_cmp::approx_eq!(f32, config.max_temperature, expected_max_temperature, epsilon=0.000001));
    }

    #[test]
    fn missing_config_file() {
        let config_file_path = "test_configs/missing_file";
        let config_reader = ConfigReader{
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
        let config_reader = ConfigReader{
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
        let config_reader = ConfigReader{
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
        let config_reader = ConfigReader{
            config_file_name: config_file_path.to_string(),
        };

        let config = config_reader.get_config();
        assert!(config.is_ok());

        let config = config.unwrap();
        assert!(config.is_none());
    }
}