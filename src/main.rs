use rocket::{
    fairing::{Fairing, Info, Kind},
    form::Form,
    http::Header,
    serde::{
        json::{serde_json::json, Json, Value},
        Serialize,
    },
    Request, Response,
};

use step::Step;
use workout::Workout;

pub mod duration;
pub mod sport;
pub mod step;
pub mod workout;

#[macro_use]
extern crate rocket;

#[get("/")]
fn get_step() -> &'static str {
    "A step!"
}

#[post("/", data = "<step>")]
fn add_step(step: Form<Step>) -> String {
    format!("{:?}", step)
}

pub struct CORS;

#[rocket::async_trait]
impl Fairing for CORS {
    fn info(&self) -> Info {
        Info {
            name: "Add CORS headers to responses",
            kind: Kind::Response,
        }
    }

    async fn on_response<'r>(&self, _request: &'r Request<'_>, response: &mut Response<'r>) {
        response.set_header(Header::new("Access-Control-Allow-Origin", "*"));
        response.set_header(Header::new(
            "Access-Control-Allow-Methods",
            "POST, GET, PATCH, OPTIONS",
        ));
        response.set_header(Header::new("Access-Control-Allow-Headers", "*"));
        response.set_header(Header::new("Access-Control-Allow-Credentials", "true"));
    }
}

#[get("/", format = "json")]
fn create_workout() -> Json<Workout> {
    let workout = Workout::default();
    Json(workout)
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(CORS)
        .mount("/step", routes![get_step, add_step])
        .mount("/workout", routes![create_workout])
}
