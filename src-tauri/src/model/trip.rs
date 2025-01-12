use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct Trip {
    name: String,
    start_date: i32,
    end_date: i32,
    total_km: i32,
    summary: String,
    cover_photo_path: String,
    all_steps: Vec<Step>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Step {
    id: i32,
    description: String,
    slug: String,
    start_time: i32,
    location: StepLocation,
    weather_condition: String,
    weather_temperatur: i32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct StepLocation {
    pub name: String,
    pub detail: String,
    pub lat: i32,
    pub lon: i32,
}
