use uno::{Card, Uno, TurnResult};

use std::io::{self, BufReader, BufRead};

fn main() {
    let players = vec![
        "Ali".into(),
        "Bob".into(),
        // "Cam".into(),
        // "Dan".into(),
        // "Edd".into(),
        // "Fil".into(),
        // "Gem".into(),
        // "Hal".into(),
    ];

    let mut uno = Uno::create_game(players);

    let stdin = io::stdin();
    let mut stdin = BufReader::new(stdin.lock());
    let mut line = String::new();
    loop {
        line.clear();
        println!("\nTurn {}", uno.current_turn());
        println!("Cards in draw pile: {}", (*uno.deck()).len());
        println!("Cards in discard pile: {}", (*uno.discard()).len());
        println!("Last card played: {}", uno.discard().peek_top_card().unwrap().display_name());
        println!("Color of last card: {:?}", uno.discard().peek_top_card().unwrap().color);
        println!("Game direction: {:?}", uno.direction());
        println!("Player {}'s turn", uno.current_player());
        println!("\nYour cards: {}", uno.current_player().get_hand());
        println!("Your move?");

        let _ = stdin.read_line(&mut line);
        line = line.trim().to_string();
        match line.trim().to_uppercase().as_ref() {
            "D" => {
                let card = uno.draw_card();
                println!("You drew a card: {}", card.display_name());
                *uno.current_player().get_hand_mut() += card;
                continue;
            },
            "EXIT" => break,
            _ => {},
        }

        match line.parse::<Card>() {
            Ok(card) => {
                println!();
                match uno.play_card(card) {
                    TurnResult::Success(c) => {
                        println!("You played a {}!", c.display_name());
                    },
                    TurnResult::InvalidMove(discard, played) => {
                        println!("Invalid move! You cannot play a {} on a {}!", played.display_name(), discard.display_name());
                    },
                    TurnResult::NotHoldingCard(c) => {
                        println!("You don't have a {}!", c.display_name());
                    },
                    TurnResult::GameOver => {
                        println!("Game over! You won!");
                        break;
                    },
                }
                println!();
            },
            Err(e) => println!("Error parsing card from {}: {}", line, e),
        }
    }

    println!("\nThanks for playing! Good bye.");
}