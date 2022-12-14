use temperature_controller::HandleTemperature;

pub mod config_reader;
pub mod temperature_controller;
pub mod temperature_modifier;
pub mod temperature_sensor;
pub mod temperature_value_provider;

fn wait_before_polling() {
    std::thread::sleep(std::time::Duration::from_millis(3000));
}
//maybe add multithreaded implementation for temperature raising
fn main() {
    let config_file_name = String::from("config.txt");
    let _config_file_reader: Box<dyn config_reader::ReadConfig> =
        Box::new(config_reader::file_reader::ConfigFileReader::new(config_file_name));

    let config_connection_string = String::from(r#"mysql://root:root@localhost:3306/thermostat"#);
    let _config_sql_reader: Box<dyn config_reader::ReadConfig> =
        Box::new(config_reader::db_reader::ConfigSqlReader::build(config_connection_string).unwrap());

    let _temperature_sensor_serial: Box<dyn temperature_sensor::FetchTemperature> =
        Box::new(temperature_sensor::serial::TemperatureSensorSerial {});
    let _temperature_sensor_http: Box<dyn temperature_sensor::FetchTemperature> =
        Box::new(temperature_sensor::http::TemperatureSensorHttp::default());

    let temperature_modifier: Box<dyn temperature_modifier::ModifyTemperature> =
        Box::new(temperature_modifier::TemperatureModifier {});
    let mut temperature_controller: temperature_controller::TemperatureController =
        temperature_controller::TemperatureController::build(
            _temperature_sensor_serial,
            temperature_modifier,
            _config_file_reader,
        );
    loop {
        temperature_controller
            .update_temperature()
            .unwrap_or_else(|err| {
                eprintln!("Something went wrong: {err}");
                std::process::exit(1);
            });
        wait_before_polling();
    }
}
