use rocket::serde::{Deserialize, Serialize};
use time::Time;

#[derive(Debug, Default, Deserialize, Serialize, PartialEq, Eq, PartialOrd, Ord, FromFormField)]
#[serde(crate = "rocket::serde")]
pub enum HeartRateZone {
    #[default]
    Zone1,
    Zone2,
    Zone3,
    Zone4,
    Zone5,
}

#[derive(Debug, Default, Deserialize, Serialize, PartialEq, Eq, PartialOrd, Ord, FromFormField)]
#[serde(crate = "rocket::serde")]
pub enum StepType {
    WarmUp,
    #[default]
    Work,
    Recover,
    Rest,
    CoolDown,
    Other,
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq, PartialOrd, Ord, FromForm)]
#[serde(crate = "rocket::serde")]
pub struct Step {
    r#type: StepType,
    duration: Option<Time>,
    target_heart_rate_zone: Option<HeartRateZone>,
    notes: Option<String>,
}

impl Default for Step {
    fn default() -> Self {
        let duration = Time::from_hms(0, 20, 0).unwrap();
        Step {
            r#type: StepType::Work,
            duration: Some(duration),
            target_heart_rate_zone: None,
            notes: None,
        }
    }
}

impl Step {
    pub fn set_type(mut self, value: StepType) -> Self {
        self.r#type = value;
        self
    }

    pub fn set_duration(mut self, value: Option<Time>) -> Self {
        self.duration = value;
        self
    }

    pub fn set_target_heart_rate_zone(mut self, value: Option<HeartRateZone>) -> Self {
        self.target_heart_rate_zone = value;
        self
    }

    pub fn set_notes(mut self, value: Option<&str>) -> Self {
        match value {
            Some(value) => {
                let string = String::from(value);
                self.notes = Some(string);
            }
            None => self.notes = None,
        }
        self
    }
}
