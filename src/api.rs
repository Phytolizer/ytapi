use google_youtube3 as yt;
use std::io::{self, BufRead, BufReader, Write};

pub fn search() {
    print!("Enter search query: ");
    io::stdout().flush().unwrap();
    let mut reader = BufReader::new(io::stdin());
    let query = {
        let mut query = String::new();
        reader.read_line(&mut query).unwrap();
        query.trim().to_string()
    };
}
pub fn view_own_playlists() {}
pub fn view_own_videos() {}
pub fn view_subscriptions() {}
pub fn view_watch_history() {}
