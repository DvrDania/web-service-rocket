#[macro_use]
extern crate rocket;

use rocket::response::status;
use rocket::serde::json::{json, Json, Value};
use rocket::State;

mod albums;
use albums::Album;

#[get("/albums")]
fn get_albums(albums: &State<Vec<Album>>) -> Json<Vec<Album>> {
    Json(albums.to_vec())
}

#[get("/albums/<id>")]
fn get_single_album(
    id: u32,
    albums: &State<Vec<Album>>,
) -> Result<Json<&Album>, status::NotFound<Value>> {
    for album in albums.iter() {
        if id == album.id {
            return Ok(Json(album));
        }
    }
    Err(status::NotFound(json!({ "message": "album not found" })))
}

#[post("/albums")]
fn add_album() {}

#[launch]
fn rocket() -> _ {
    let albums = albums::dummy();
    rocket::build()
        .manage(albums)
        .mount("/", routes![get_albums, get_single_album, add_album])
}
