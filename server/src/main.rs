mod database;

use std::path::Path;

use rocket::{
    launch, 
    fs::{FileServer, NamedFile}, 
    catch, 
    catchers
};

use database::establish_connection;

#[catch(404)]
async fn fallback_to_index() -> Option<NamedFile> {
    NamedFile::open(Path::new("../ui/dist/index.html")).await.ok()
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .manage(establish_connection())
        .register("/", catchers![fallback_to_index])
        .mount("/", FileServer::from("../ui/dist"))
}
