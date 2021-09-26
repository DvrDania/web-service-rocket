use rocket::serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(crate = "rocket::serde")]
pub struct Album {
    pub id: u32,
    pub title: String,
    pub artist: String,
    pub price: f32,
}

pub fn dummy() -> Vec<Album> {
    let albums = vec![
        Album {
            id: 1,
            title: String::from("Blue Train"),
            artist: String::from("John Coltrane"),
            price: 56.99,
        },
        Album {
            id: 2,
            title: String::from("Jeru"),
            artist: String::from("Gerry Mulligan"),
            price: 17.99,
        },
        Album {
            id: 3,
            title: String::from("Sarah Vaughan and Clifford Brown"),
            artist: String::from("Sarah Vaughan"),
            price: 39.99,
        },
    ];
    albums
}
