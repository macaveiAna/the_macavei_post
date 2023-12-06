/**
 * src/pages/music.rs
 * This file handles the music content
 *
*/
use actix_web::{web, HttpResponse};
use handlebars::Handlebars;
use serde_json::json;

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
        let body = hb.render("music", &music_content).unwrap();
        HttpResponse::Ok().body(body)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::{http, test, App};

    #[tokio::test]
    async fn test_music_handler() {
        //create app with music rout
        let mut app =
            test::init_service(App::new().route("/music", web::get().to(HandleMusic::music))).await;

        let req = test::TestRequest::get().uri("/music").to_request();
        let res = test::call_service(&mut app, req).await;

        assert_eq!(res.status(), http::StatusCode::OK);

        let body = test::read_body(res).await;
        /*assert!(
            !body
                .iter()
                .zip("error".as_bytes().iter())
                .any(|(a, b)| a != b),
            "Response body contains 'error'"
        );*/

        assert!(body
            .windows("Yes! (feat. Jack Liebeck)".len())
            .any(|window| window == "Yes! (feat. Jack Liebeck)".as_bytes()));
        assert!(body
            .windows("Dario Marianelli, Jack Liebeck, Benjamin Wallfisch".len())
            .any(
                |window| window == "Dario Marianelli, Jack Liebeck, Benjamin Wallfisch".as_bytes()
            ));
        assert!(body
            .windows(
                "https://open.spotify.com/track/2AhTppjasVXtLuMGZkoXyV?si=88cbe7efb6f54a3c".len()
            )
            .any(|window| window
                == "https://open.spotify.com/track/2AhTppjasVXtLuMGZkoXyV?si=88cbe7efb6f54a3c"
                    .as_bytes()));
    }
}
