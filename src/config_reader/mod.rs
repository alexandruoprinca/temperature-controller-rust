use std::io::BufRead;

use mysql::prelude::Queryable;

#[derive(Clone, Copy)]
pub struct Config {
    pub min_temperature: f32,
    pub max_temperature: f32,
}

#[mockall::automock]
pub trait ReadConfig {
    fn get_config(&self) -> Result<Option<Config>, Box<dyn std::error::Error>>;
}

pub struct ConfigFileReader {
    config_file_name: String,
}

pub struct ConfigSqlReader {
    pool: mysql::Pool,
}

impl ConfigFileReader {
    pub fn new(config_file_name: String) -> Self {
        ConfigFileReader { config_file_name }
    }
}

impl ConfigSqlReader {
    pub fn build(sql_connection_string: String) -> Result<Self, Box<dyn std::error::Error>> {
        let opts = mysql::Opts::from_url(&sql_connection_string)?;
        let pool = mysql::Pool::new(opts)?;
        Ok(ConfigSqlReader { pool })
    }
}

fn extract_config_from_line(line: &str) -> Option<Config> {
    println!("Receiving config {}", line);
    let values = line.split_whitespace().collect::<Vec<&str>>();
    if values.len() < 2 {
        eprintln!("Did not enter correct config values");
        return None;
    }

    let min_temperature = match values[0].trim().parse::<f32>() {
        Ok(result) => result,
        Err(err) => {
            eprintln!("Receiving data {line}");
            eprintln!("Failed to convert min temp to a number. Received min temp is {err}");
            return None;
        }
    };

    let max_temperature = match values[1].trim().parse::<f32>() {
        Ok(result) => result,
        Err(err) => {
            eprintln!("Receiving data {line}");
            eprintln!("Failed to convert max temp to a number. Received max temp is {err}");
            return None;
        }
    };

    Some(Config {
        min_temperature,
        max_temperature,
    })
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

impl ReadConfig for ConfigSqlReader {
    fn get_config(&self) -> Result<Option<Config>, Box<dyn std::error::Error>> {
        let mut conn = self.pool.get_conn()?;
        let result = conn
            .query_map(
                r#"SELECT min_temperature, max_temperature FROM Config"#,
                |(min_temperature, max_temperature)| Config {
                    min_temperature,
                    max_temperature,
                },
            )?;
        if result.len() != 0 {
            return Ok(Some(result[0]));
        }
        Ok(None)
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
