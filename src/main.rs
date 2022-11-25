use temperature_controller::HandleTemperature;

pub mod temperature_sensor;
pub mod config_reader;
pub mod temperature_modifier;
pub mod temperature_value_provider;
pub mod temperature_controller;

fn wait_before_polling() {
    std::thread::sleep(std::time::Duration::from_millis(3000));
}

fn main() {
    let config_file_name = String::from("config.txt");
    let config_file_reader: Box::<dyn config_reader::ReadConfig> = Box::new(config_reader::ConfigFileReader::new(config_file_name));

    let config_connection_string = String::from(r#"mysql://root:root@localhost:3306/thermostat"#);
    let config_sql_reader: Box::<dyn config_reader::ReadConfig> = Box::new(config_reader::ConfigSqlReader::build(config_connection_string).unwrap());

    let temperature_sensor: Box::<dyn temperature_sensor::FetchTemperature> = Box::new(temperature_sensor::TemperatureSensor{});
    let temperature_modifier: Box::<dyn temperature_modifier::ModifyTemperature> = Box::new(temperature_modifier::TemperatureModifier{});
    let mut temperature_controller: temperature_controller::TemperatureController = temperature_controller::TemperatureController::build(temperature_sensor, temperature_modifier, config_file_reader);
    loop {
        temperature_controller.update_temperature().unwrap_or_else(|err| {
            eprintln!("Something went wrong: {err}");
            std::process::exit(1);
        });
        wait_before_polling();
    }
}
