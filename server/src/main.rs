mod database;

use rocket::{launch, get, routes};
use database::establish_connection;

#[get("/")]
fn index() -> &'static str {
    "Hello, World!"
}

#[launch]
fn rocket() -> _ {
    rocket::build().manage(establish_connection()).mount("/", routes![index])
}
