#[macro_use]
extern crate rocket;

use rocket::response::status;
use rocket::serde::json::{json, Json, Value};
use rocket::State;
use std::sync::Arc;
use std::sync::Mutex;

mod albums;
use albums::Album;

#[get("/albums")]
fn get_albums(albums: &State<Arc<Mutex<Vec<Album>>>>) -> Json<Vec<Album>> {
    Json(albums.lock().unwrap().to_vec())
}

#[get("/albums/<id>")]
fn get_single_album(
    id: u32,
    albums: &State<Arc<Mutex<Vec<Album>>>>,
) -> Result<Json<Album>, status::NotFound<Value>> {
    let albums = albums.lock().unwrap().to_vec();
    for album in albums {
        if id == album.id {
            return Ok(Json(album));
        }
    }
    Err(status::NotFound(json!({ "message": "album not found" })))
}

#[post("/albums", format = "json", data = "<new>")]
fn add_album(new: Json<Album>, albums: &State<Arc<Mutex<Vec<Album>>>>) {
    albums.lock().unwrap().push(new.into_inner());
}

#[launch]
fn rocket() -> _ {
    let albums = Arc::from(Mutex::from(albums::dummy()));
    rocket::build()
        .manage(albums)
        .mount("/", routes![get_albums, get_single_album, add_album])
}
