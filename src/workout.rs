use rocket::serde::Serialize;

use crate::sport::Sport;
use crate::step::{Step, StepType};

use rocket::FromForm;

#[derive(Debug, Serialize, PartialEq, Eq, PartialOrd, Ord, FromForm)]
#[serde(crate = "rocket::serde")]
pub struct Workout {
    sport: Sport,
    name: Option<String>,
    steps: Vec<Step>,
    note: Option<String>,
}

impl Default for Workout {
    fn default() -> Self {
        let warm_up_step = Step::default()
            .set_type(StepType::WarmUp)
            .set_duration(None);
        let work_step = Step::default();
        let cool_down_step = Step::default()
            .set_type(StepType::CoolDown)
            .set_duration(None);
        Workout {
            sport: Sport::Cycling,
            name: None,
            steps: vec![warm_up_step, work_step, cool_down_step],
            note: None,
        }
    }
}
