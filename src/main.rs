mod api;

use api::search;
use api::view_own_playlists;
use api::view_own_videos;
use api::view_subscriptions;
use api::view_watch_history;
use google_youtube3 as yt;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct SecretFile {
    installed: Secret,
}

#[derive(Debug, Deserialize)]
struct Secret {
    client_id: String,
    auth_uri: String,
    token_uri: String,
    client_secret: String,
    redirect_uris: Vec<String>,
}

fn main() {
    let secret_file: SecretFile =
        serde_json::from_str(std::fs::read("client_secret.json").unwrap())
            .unwrap();
    let secret = secret_file.installed;
    dbg(secret);
    let mut hub = yt::YouTube::new(reqwest::Client::new(), auth);
    let main_menu = dialoguer::Select::new().items(&[
        "View subscriptions",
        "Search for a video or playlist",
        "Your videos",
        "Your playlists",
        "Your watch history",
        "Quit the application",
    ]);
    loop {
        match main_menu.interact().unwrap() {
            0 => view_subscriptions(),
            1 => search(),
            2 => view_own_videos(),
            3 => view_own_playlists(),
            4 => view_watch_history(),
            5 => break,
            _ => unreachable!(),
        }
    }
}
