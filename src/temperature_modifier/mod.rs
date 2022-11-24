use crate::temperature_value_provider::TemperatureValueProvider;

#[mockall::automock]
pub trait ModifyTemperature {
    fn lower_temperature(&self, target_temperature: f32) -> Result<(), &'static str>;
    fn raise_temperature(&self, target_temperature: f32) -> Result<(), &'static str>;
}

pub struct TemperatureModifier{}

impl ModifyTemperature for TemperatureModifier {
    fn lower_temperature(&self, target_temperature :f32) -> Result<(), &'static str> {
        let mut current_temperature: f32;
        loop {
            current_temperature = TemperatureValueProvider::get_current_temperature();
            if current_temperature > target_temperature {
                TemperatureValueProvider::set_current_temperature(current_temperature - 1.0);
                std::thread::sleep(std::time::Duration::from_millis(100));
            } else {
                return Ok(());
            }
        }
    }

    fn raise_temperature(&self, target_temperature :f32) -> Result<(), &'static str> {
        let mut current_temperature: f32;
        loop {
            current_temperature = TemperatureValueProvider::get_current_temperature();
            if current_temperature < target_temperature {
                TemperatureValueProvider::set_current_temperature(current_temperature + 1.0);
                std::thread::sleep(std::time::Duration::from_millis(100));
            } else {
                return Ok(());
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn raise_above_current() {
        let temperature_modifier = TemperatureModifier{};
        let initial_temperature = 5 as f32;
        TemperatureValueProvider::set_current_temperature(initial_temperature);
        let expected_temperature = 10 as f32;
        let target_temperature = 10 as f32;

        let temperature_was_modified = temperature_modifier.raise_temperature(target_temperature);

        assert!(temperature_was_modified.is_ok());
        let current_temperature = TemperatureValueProvider::get_current_temperature();
        assert!(float_cmp::approx_eq!(f32, current_temperature, expected_temperature, epsilon=0.000001));
    }

    #[test]
    fn raise_below_current() {
        let temperature_modifier = TemperatureModifier{};
        let initial_temperature = 5 as f32;
        TemperatureValueProvider::set_current_temperature(initial_temperature);
        let expected_temperature = 5 as f32;
        let target_temperature = 3 as f32;

        let temperature_was_modified = temperature_modifier.raise_temperature(target_temperature);

        assert!(temperature_was_modified.is_ok());
        let current_temperature = TemperatureValueProvider::get_current_temperature();
        assert!(float_cmp::approx_eq!(f32, current_temperature, expected_temperature, epsilon=0.000001));
    }

    #[test]
    fn lower_below_current() {
        let temperature_modifier = TemperatureModifier{};
        let initial_temperature = 5 as f32;
        TemperatureValueProvider::set_current_temperature(initial_temperature);
        let expected_temperature = 3 as f32;
        let target_temperature = 3 as f32;

        let temperature_was_modified = temperature_modifier.lower_temperature(target_temperature);

        assert!(temperature_was_modified.is_ok());
        let current_temperature = TemperatureValueProvider::get_current_temperature();
        assert!(float_cmp::approx_eq!(f32, current_temperature, expected_temperature, epsilon=0.000001));
    }

    #[test]
    fn lower_above_current() {
        let temperature_modifier = TemperatureModifier{};
        let initial_temperature = 5 as f32;
        TemperatureValueProvider::set_current_temperature(initial_temperature);
        let expected_temperature = 5 as f32;
        let target_temperature = 10 as f32;

        let temperature_was_modified = temperature_modifier.lower_temperature(target_temperature);

        assert!(temperature_was_modified.is_ok());
        let current_temperature = TemperatureValueProvider::get_current_temperature();
        assert!(float_cmp::approx_eq!(f32, current_temperature, expected_temperature, epsilon=0.000001));
    }
}