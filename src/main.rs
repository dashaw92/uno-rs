#![allow(unused)]

mod card;
mod deck;
mod uno;
mod direction;
mod player;

use crate::player::Player;
use crate::uno::Uno;

fn main() {
    let players = vec![
        Player::new("Dan"),
        Player::new("Ali"),
        Player::new("Bob"),
        Player::new("Carl"),
    ];

    let uno = Uno::create_game(players);
    println!("{}", uno);
}