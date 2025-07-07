use rand::seq::SliceRandom;
use serde::Deserialize;
use std::fs;

#[derive(Deserialize, Debug, Clone)]
pub struct Card { pub id: u32, pub title: String, pub text: String}

pub fn draw() -> Card {
    let raw = fs::read_to_string("data/inspiration_deck.json").expect("Deck Missing");
    let mut deck: Vec<Card> = serde_json::from_str(&raw).unwrap();
    deck.shuffle(&mut rand::thread_rng());
    deck[0].clone()
}