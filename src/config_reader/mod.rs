use std::io::BufRead;

pub struct Config {
    pub min_temperature: f32,
    pub max_temperature: f32,
}

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
            eprintln!("Did not correct config values");
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

      //  buffer_reader.read_line(&mut buffer)?;
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
