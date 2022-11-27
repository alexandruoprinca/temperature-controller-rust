pub mod serial;
pub mod http;
#[mockall::automock]
pub trait FetchTemperature {
    fn get_current_temperature(&self) -> Option<f32>;
}