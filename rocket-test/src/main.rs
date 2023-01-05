use rand::{self, Rng};
use rocket::{ get, launch, serde::json::Json};

#[macro_use]
extern crate rocket;

#[get("/")]
async fn index() -> &'static str {
    "Hello, world!"
}

#[get("/rand")]
async fn get_rand() -> Json<f32> {
    Json(rand::thread_rng().gen::<f32>())
}

#[launch]
fn rocket() -> _ {
    let figment = rocket::Config::figment().merge(("log_level", "off"));

    rocket::custom(figment).mount("/", routes![index, get_rand])
}
