use crate::temperature_value_provider::TemperatureValueProvider;

#[mockall::automock]
pub trait FetchTemperature {
    fn get_current_temperature(&self) -> Option<f32>;
}

pub struct TemperatureSensor {
}

impl FetchTemperature for TemperatureSensor {
    fn get_current_temperature(&self) -> Option<f32> {
        return Some(TemperatureValueProvider::get_current_temperature());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fetch_temperature() {
        TemperatureValueProvider::set_current_temperature(5.0);
        let expected_temperature: f32 = 5f32;
        let sensor = TemperatureSensor{};
        assert!(float_cmp::approx_eq!(f32, sensor.get_current_temperature().unwrap(), expected_temperature, epsilon=0.000001));
    }
}