use super::http;
use {Profile, Leaderboard};

pub fn leaderboard(mode: &str, start: Option<usize>, end: Option<usize>) -> Leaderboard {

    let url;
    match start {
        None => url = format!("https://api.rocco.dev/{}/winstreaks/leaderboard/", mode),
        Some(ref start) => url = format!("https://api.rocco.dev/{}/winstreaks/leaderboard?from={}&to={}", mode, start, end.unwrap_or(500)),
    }
    let json = http::json_from_url(url);
    return json;
        
}

pub fn profile(mode: &str, uuid: &str) -> Profile {
    let url = format!("https://api.rocco.dev/{}/winstreaks/profile/{}", mode, uuid);
    let json = http::json_from_url(url);
    return json; 
}

