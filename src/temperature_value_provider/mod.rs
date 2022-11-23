use rand::Rng;
lazy_static::lazy_static! {
static ref TEMPERATURE: std::sync::Mutex<f32> = std::sync::Mutex::new(generate_random_temperature());
}

fn generate_random_temperature() -> f32 {
    rand::thread_rng().gen_range(-35..35) as f32
}

pub struct TemperatureValueProvider {
}

impl TemperatureValueProvider{
    pub fn get_current_temperature() -> f32{
        return *TEMPERATURE.lock().unwrap();
    }

    pub fn set_current_temperature(temp: f32) {
        *TEMPERATURE.lock().unwrap() = temp;
    }
}
