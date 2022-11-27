use crate::{temperature_sensor::FetchTemperature, temperature_value_provider::TemperatureValueProvider};

pub struct TemperatureSensorSerial {}

impl FetchTemperature for TemperatureSensorSerial {
    fn get_current_temperature(&self) -> Option<f32> {
        return Some(TemperatureValueProvider::get_current_temperature());
    }
}

mod tests {
    use super::*;
    
    #[test]
    fn fetch_temperature() {
        TemperatureValueProvider::set_current_temperature(5.0);
        let expected_temperature: f32 = 5f32;
        let sensor = TemperatureSensorSerial {};
        assert!(float_cmp::approx_eq!(
            f32,
            sensor.get_current_temperature().unwrap(),
            expected_temperature,
            epsilon = 0.000001
        ));
    }
}