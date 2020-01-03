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
        "Dan".into(),
        "Ali".into(),
        "Bob".into(),
        "Carl".into(),
    ];

    let uno = Uno::create_game(players);
    println!("{}", uno);
}