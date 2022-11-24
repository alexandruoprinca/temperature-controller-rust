use temperature_controller::{TemperatureController, HandleTemperature};

pub mod temperature_sensor;
pub mod config_reader;
pub mod temperature_modifier;
pub mod temperature_value_provider;
pub mod temperature_controller;

fn wait_before_polling() {
    std::thread::sleep(std::time::Duration::from_millis(2000));
}

fn main() {
    let config_file_name = String::from("config.txt");
    let config_reader: Box::<dyn config_reader::ParseConfig> = Box::new(config_reader::ConfigReader::new(config_file_name));
    let temperature_sensor: Box::<dyn temperature_sensor::FetchTemperature> = Box::new(temperature_sensor::TemperatureSensor{});
    let temperature_modifier: Box::<dyn temperature_modifier::ModifyTemperature> = Box::new(temperature_modifier::TemperatureModifier{});
    let mut temperature_controller: TemperatureController = TemperatureController::build(temperature_sensor, temperature_modifier, config_reader);
    loop {
        temperature_controller.update_temperature().unwrap_or_else(|err| {
            eprintln!("Something went wrong: {err}");
            std::process::exit(1);
        });
        wait_before_polling();
    }
}
