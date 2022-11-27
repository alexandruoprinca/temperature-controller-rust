pub mod file_reader;
pub mod db_reader;

#[derive(Clone, Copy)]
pub struct Config {
    pub min_temperature: f32,
    pub max_temperature: f32,
}

#[mockall::automock]
pub trait ReadConfig {
    fn get_config(&self) -> Result<Option<Config>, Box<dyn std::error::Error>>;
}

fn extract_config_from_line(line: &str) -> Option<Config> {
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

