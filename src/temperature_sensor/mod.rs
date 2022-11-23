use crate::temperature_value_provider::TemperatureValueProvider;

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