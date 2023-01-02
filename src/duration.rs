use rocket::serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub enum DistanceUnits {
    Miles,
    Kilometers,
    Meters,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub enum Duration {
    Time(u32),
    Distance { units: DistanceUnits, value: u32 },
    LapButtonPress,
    Calories(u32),
    HeartRate { above: bool, bpm: u32 },
    Power { above: bool, watts: u32 },
}

impl Default for Duration {
    fn default() -> Self {
        Duration::Time(20 * 60)
    }
}
