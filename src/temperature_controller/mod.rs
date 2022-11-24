use crate::config_reader::ParseConfig;
use crate::temperature_modifier::ModifyTemperature;
use crate::temperature_sensor::FetchTemperature;

pub trait HandleTemperature {
    fn update_temperature(&mut self) -> Result<(), String>;
    fn get_current_state(&self) -> &SystemState;
}

pub struct TemperatureController {
    sensor: Box<dyn FetchTemperature>,
    temperature_modifier: Box<dyn ModifyTemperature>,
    config_reader: Box<dyn ParseConfig>,
    current_state: SystemState,
}

impl TemperatureController {
    pub fn build(
        sensor: Box<dyn FetchTemperature>,
        temperature_modifier: Box<dyn ModifyTemperature>,
        config_reader: Box<dyn ParseConfig>,
    ) -> Self {
        TemperatureController {
            sensor,
            temperature_modifier,
            config_reader,
            current_state: SystemState::Idle,
        }
    }

    fn change_system_state(&mut self, new_state: SystemState) {
        self.current_state = new_state;
    }
}

impl HandleTemperature for TemperatureController {
    fn get_current_state(&self) -> &SystemState {
        &self.current_state
    }

    //maybe this should not be String, figure out
    fn update_temperature(&mut self) -> Result<(), String> {
        let config = match self.config_reader.get_config() {
            Ok(result) => result,
            Err(err) => return Err(std::format!("Failed to read config file: {err}")),
        };
        let config = match config {
            Some(val) => val,
            None => return Err(std::format!("Failed to parse config")),
        };
        let current_temperature = match self.sensor.get_current_temperature() {
            Some(val) => val,
            None => return Err(std::format!("Failed to read sensor data")),
        };

        match self.current_state {
            SystemState::Idle => {
                if config.min_temperature >= current_temperature {
                    println!("Current temperature {current_temperature} is equal or lower than minimum value of {}, raising temperature", config.min_temperature);
                    self.change_system_state(SystemState::Heating);
                } else if config.max_temperature <= current_temperature {
                    println!("Current temperature {current_temperature} is equal or higher than maximum value of {}, lowering temperature", config.max_temperature);
                    self.change_system_state(SystemState::Cooling);
                } else {
                    println!(
                        "Temperature {current_temperature} is withing parameters {} and {}",
                        config.min_temperature, config.max_temperature
                    );
                }
            }
            SystemState::Cooling => {
                if self
                    .temperature_modifier
                    .lower_temperature(config.max_temperature - 1.0)
                    .is_err()
                {
                    eprintln!("Failed to cool temperature");
                } else {
                    println!("Lowered temperature");
                    self.change_system_state(SystemState::Idle);
                }
            }
            SystemState::Heating => {
                if self
                    .temperature_modifier
                    .raise_temperature(config.min_temperature + 1.0)
                    .is_err()
                {
                    eprintln!("Failed to raise temperature");
                } else {
                    println!("Raised temperature");
                    self.change_system_state(SystemState::Idle);
                }
            }
        };
        Ok(())
    }
}

#[derive(PartialEq)]
pub enum SystemState {
    Idle,
    Heating,
    Cooling,
}

#[cfg(test)]
mod tests {
    use crate::{
        config_reader::{self, Config},
        temperature_modifier, temperature_sensor,
    };

    use super::*;

    #[test]
    fn temperature_in_parameters() {
        let mut config_reader_mock = Box::new(config_reader::MockParseConfig::new());
        let mut temperature_sensor_mock = Box::new(temperature_sensor::MockFetchTemperature::new());
        let temperature_modifier_mock =
            Box::new(temperature_modifier::MockModifyTemperature::new());

        config_reader_mock.expect_get_config().returning(|| {
            Ok(Some(Config {
                min_temperature: -5f32,
                max_temperature: 10f32,
            }))
        });
        temperature_sensor_mock
            .expect_get_current_temperature()
            .returning(|| Some(5f32));

        let mut temperature_controller: TemperatureController = TemperatureController::build(
            temperature_sensor_mock,
            temperature_modifier_mock,
            config_reader_mock,
        );

        let temperature_updated = temperature_controller.update_temperature();
        assert!(temperature_updated.is_ok());
    }

    #[test]
    fn temperature_below_parameters() {
        let mut config_reader_mock = Box::new(config_reader::MockParseConfig::new());
        let mut temperature_sensor_mock = Box::new(temperature_sensor::MockFetchTemperature::new());
        let mut temperature_modifier_mock =
            Box::new(temperature_modifier::MockModifyTemperature::new());

        config_reader_mock.expect_get_config().returning(|| {
            Ok(Some(Config {
                min_temperature: -5f32,
                max_temperature: 10f32,
            }))
        });
        temperature_sensor_mock
            .expect_get_current_temperature()
            .returning(|| Some(-7f32));

        temperature_modifier_mock.expect_raise_temperature().returning(|_| Ok(()));
        
        let mut temperature_controller: TemperatureController = TemperatureController::build(
            temperature_sensor_mock,
            temperature_modifier_mock,
            config_reader_mock,
        );

        let temperature_updated = temperature_controller.update_temperature();
        assert!(temperature_updated.is_ok());
        let expected_state = SystemState::Heating;
        let current_state = temperature_controller.get_current_state();
        assert!(*current_state == expected_state);

        let temperature_updated = temperature_controller.update_temperature();
        assert!(temperature_updated.is_ok());
        let expected_state = SystemState::Idle;
        let current_state = temperature_controller.get_current_state();
        assert!(*current_state == expected_state);

    }

    #[test]
    fn temperature_above_parameters() {
        let mut config_reader_mock = Box::new(config_reader::MockParseConfig::new());
        let mut temperature_sensor_mock = Box::new(temperature_sensor::MockFetchTemperature::new());
        let mut temperature_modifier_mock =
            Box::new(temperature_modifier::MockModifyTemperature::new());

        config_reader_mock.expect_get_config().returning(|| {
            Ok(Some(Config {
                min_temperature: -5f32,
                max_temperature: 10f32,
            }))
        });
        temperature_sensor_mock
            .expect_get_current_temperature()
            .returning(|| Some(15f32));
            
        temperature_modifier_mock.expect_lower_temperature().returning(|_| Ok(()));
        
        let mut temperature_controller: TemperatureController = TemperatureController::build(
            temperature_sensor_mock,
            temperature_modifier_mock,
            config_reader_mock,
        );

        let temperature_updated = temperature_controller.update_temperature();
        assert!(temperature_updated.is_ok());
        let expected_state = SystemState::Cooling;
        let current_state = temperature_controller.get_current_state();
        assert!(*current_state == expected_state);

        let temperature_updated = temperature_controller.update_temperature();
        assert!(temperature_updated.is_ok());
        let expected_state = SystemState::Idle;
        let current_state = temperature_controller.get_current_state();
        assert!(*current_state == expected_state);
    }
}
