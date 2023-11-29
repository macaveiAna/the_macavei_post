/**
 * src/pages/music.rs
 * This file handles the music content
 * 
*/

use actix_web::{web, HttpResponse};
use serde_json::json;
use handlebars::Handlebars;

pub struct HandleMusic;

// A function that handles the list of my favorite songs
impl HandleMusic {
    pub async fn music(hb: web::Data<Handlebars<'_>>) -> HttpResponse {
        //my logic for the music page
        // try to find another way to access this content
        let music_content = json!({
            "content": [
                {
                    "name": "Yes! (feat. Jack Liebeck)",
                    "artist": "Dario Marianelli, Jack Liebeck, Benjamin Wallfisch",
                    "spotify_link": "https://open.spotify.com/track/2AhTppjasVXtLuMGZkoXyV?si=88cbe7efb6f54a3c"

                },
                {
                    "name": "Meant To Live (Jon Bellion Version)",
                    "artist": "Jon Bellion",
                    "spotify_link": "https://open.spotify.com/track/3ClVeAl8mEEptUXDJConMo?si=a72f710e88a44652"
                },
                {
                    "name": "In My Blood",
                    "artist": "Switchfoot",
                    "spotify_link": "https://open.spotify.com/track/1f7qZrOScGlIcjWpfRrmSI?si=ea59bacea7ed4bbd"

                },
                {
                    "name": "The Burning Bush",
                    "artist":"Hans Zimmer",
                    "spotify_link":"https://open.spotify.com/track/5tgyHc2LLF0BI2udBTMFJz?si=5574abea42d44d87"
                }

            ] 
        });
        let body =hb.render("music", &music_content).unwrap();
        HttpResponse::Ok().body(body)
    }
}