extern crate reqwest;
extern crate scraper;

use scraper::{Html, Selector};

fn main() {
    let base_url = "https://gg.deals/games/";
    let verified_games = get_verified_games(base_url);
    let playable_games = get_playable_games(base_url);
    println!("{:?}", verified_games.len());
    println!("{:?}", playable_games.len());
}

fn get_verified_games(base_url: &str) -> Vec<String> {
    let mut i = 1;
    let mut games = Vec::new();
    loop {
        let page_url = format!(
            "{}?page={}",
            format!("{}steam-deck-verified/", base_url),
            &i.to_string()
        );
        let page_games = get_games_on_site(&page_url);
        if page_games.is_empty() {
            break;
        }
        games.extend(page_games);
        i += 1;
    }
    games
}

fn get_playable_games(base_url: &str) -> Vec<String> {
    let mut i = 1;
    let mut games = Vec::new();
    loop {
        let page_url = format!(
            "{}?page={}",
            format!("{}steam-deck-playable/", base_url),
            &i.to_string()
        );
        let page_games = get_games_on_site(&page_url);
        if page_games.is_empty() {
            break;
        }
        games.extend(page_games);
        i += 1;
    }
    games
}

fn get_games_on_site(url: &str) -> Vec<String> {
    let req = reqwest::blocking::get(url).unwrap();

    let body = Html::parse_document(&req.text().unwrap());

    let selector = Selector::parse("a.game-info-title").unwrap();

    let mut games = Vec::new();
    for node in body.select(&selector) {
        games.push(node.text().collect::<Vec<_>>()[0].to_string());
    }

    games
}
