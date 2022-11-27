use crate::temperature_sensor::FetchTemperature;

pub struct TemperatureSensorHttp {
    resource_path: &'static str,
}

impl Default for TemperatureSensorHttp {
    fn default() -> TemperatureSensorHttp {
        TemperatureSensorHttp {
            resource_path: "http://127.0.0.1:8000/temperature",
        }
    }
}

impl FetchTemperature for TemperatureSensorHttp {
    fn get_current_temperature(&self) -> Option<f32> {
        let temperature = match reqwest::blocking::get(self.resource_path) {
            Ok(val) => val,
            Err(val) => {
                eprintln!(
                    "Failed to get data from resource {} : {val}",
                    self.resource_path
                );
                return None;
            }
        };
        if !temperature.status().is_success() {
            eprintln!("Request failed with error {}", temperature.status());
            return None;
        }
        let temperature: serde_json::Value = match temperature.json() {
            Ok(val) => val,
            Err(err) => {
                eprintln!("Failed to parse json: {err}");
                return None;
            }
        };
        let temperature = match temperature.get("temperature") {
            Some(val) => match val.as_str() {
                Some(temp) => temp,
                None => {
                    eprintln!("Failed to convert value to string");
                    return None;
                }
            },
            None => {
                eprintln!("Value temperature does not exist");
                return None;
            }
        };
        match temperature.parse::<f32>() {
            Ok(val) => {
                return Some(val);
            }
            Err(err) => {
                eprintln!("Failed to parse string to f32: {err}");
                return None;
            }
        }
    }
}