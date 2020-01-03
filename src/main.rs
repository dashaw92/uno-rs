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
        "Ali".into(),
        "Bob".into(),
        "Cam".into(),
        "Dan".into(),
    ];

    let uno = Uno::create_game(players);
    println!("{}", uno);
}