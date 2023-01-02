use rocket::serde::Serialize;

#[derive(Debug, Serialize, PartialEq, Eq, PartialOrd, Ord, Default, FromFormField)]
#[serde(crate = "rocket::serde")]
pub enum Sport {
    #[default]
    Cycling,
    Running,
    Swimming,
    CardioTraining,
    StrengthTraining,
    Yoga,
    Pilates,
    Custom,
}
