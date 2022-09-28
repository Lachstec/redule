mod database;

use rocket::{launch, fs::FileServer, get, routes};
use database::establish_connection;

#[get("/")]
fn hello_world() -> &'static str {
    "hello, world!"
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .manage(establish_connection())
        .mount("/", FileServer::from("../ui/dist"))
        .mount("/api", routes![hello_world])
}
