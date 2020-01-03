#![allow(unused)]

mod card;
mod deck;
mod uno;
mod direction;
mod player;

use crate::card::*;
use crate::player::Player;
use crate::uno::Uno;

use std::io::{self, Read, BufReader, BufRead};

fn main() {
    let players = vec![
        "Ali".into(),
        "Bob".into(),
        "Cam".into(),
        "Dan".into(),
    ];

    let mut uno = Uno::create_game(players);

    let stdin = io::stdin();
    let mut stdin = BufReader::new(stdin.lock());
    let mut line = String::new();
    loop {
        println!("{}", uno);
        println!("\nPlayer {}'s turn", uno.current_player());
        println!("Your move?");

        stdin.read_line(&mut line);
        match line.parse::<CardType>() {
            Ok(card) => {
                println!();
                println!("You played a {}!", card);
                println!("{:?}", uno.play_card(card));
                println!();
            }
            Err(e) => println!("Error parsing card from input: {}", e),
        }

        line.clear();
    }
}