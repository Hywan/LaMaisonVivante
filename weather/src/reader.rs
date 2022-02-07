use crate::state::*;
use reqwest::Result;

pub fn read(openweathermap_api_key: &str) -> Result<State> {
    reqwest::blocking::get(format!(
        "https://api.openweathermap.org/data/2.5/onecall?appid={openweathermap_api_key}&lat={latitude}&lon={longitude}&units=metric&lang={language}&exclude=daily,minutely",
        openweathermap_api_key = openweathermap_api_key,
        latitude = HOME_LATITUDE,
        longitude = HOME_LONGITUDE,
        language = HOME_LANGUAGE,
    ))?
        .json::<State>()
}
