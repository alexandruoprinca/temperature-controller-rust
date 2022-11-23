use crate::temperature_value_provider::TemperatureValueProvider;

pub trait ModifyTemperature {
    fn lower_temperature(&self, target_temperature: f32) -> Result<(), &str>;
    fn raise_temperature(&self, target_temperature: f32) -> Result<(), &'static str>;
}

pub struct TemperatureModifier{}

impl ModifyTemperature for TemperatureModifier {
    fn lower_temperature(&self, target_temperature :f32) -> Result<(), &str> {
        let mut current_temperature: f32;
        loop {
            current_temperature = TemperatureValueProvider::get_current_temperature();
            if current_temperature > target_temperature {
                println!("Received current temperature {current_temperature} and target {target_temperature}");
                TemperatureValueProvider::set_current_temperature(current_temperature - 1.0);
                std::thread::sleep(std::time::Duration::from_millis(300));
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
                std::thread::sleep(std::time::Duration::from_millis(300));
            } else {
                return Ok(());
            }
        }
    }
}