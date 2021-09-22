#[macro_use]
extern crate rocket;

use rocket::serde::{json::Json, Deserialize, Serialize};
use serde_json::{from_str, Value};
use std::fs::read_to_string;

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
struct Album {
    id: u32,
    title: String,
    artist: String,
    price: f32,
}

#[get("/albums")]
fn get_albums() -> Json<Vec<Album>> {
    let json = read_to_string("albums.json").unwrap();
    let json: Value = from_str(&json).unwrap();
    let albums: Vec<Album> = serde_json::from_value(json["albums"].clone()).unwrap();
    Json(albums)
}

#[get("/albums/<id>")]
fn get_single_album(id: u32) {
    println!("{}", id);
}

#[post("/albums")]
fn add_album() {}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![get_albums, get_single_album, add_album])
}
