use serde::{Serialize, Deserialize};
#[derive(Serialize, Deserialize)]
pub struct Request {
    endpoint: String,
    format_str: String,
}

pub enum RequestObject {
    AirQualityRequest(AirQuality),
}

#[derive(Serialize, Deserialize)]
pub struct AirQuality {
    overall_aqi: usize,
    CO: AirQualityField,
    PM10: AirQualityField,
    SO2: AirQualityField,
    #[serde(rename(deserialize = "PM2.5"))]
    PM2p5: AirQualityField,
}
 
#[derive(Serialize, Deserialize)]
pub struct AirQualityField {
    concentration: f64,
    aqi: usize
}


