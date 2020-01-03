use uno::*;

use std::io::{self, BufReader, BufRead};

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
        line.clear();
        println!("\n{}", uno);
        println!("\nPlayer {}'s turn", uno.current_player());
        println!("Your cards: {}", uno.current_player().hand);
        println!("Your move?");

        let _ = stdin.read_line(&mut line);
        match line.trim().to_uppercase().as_ref() {
            "D" => {
                let card = uno.draw_card();
                println!("You drew a card: {}", card.display_name());
                uno.current_player().hand += card;
                continue;
            },
            "EXIT" => break,
            _ => {},
        }

        match line.parse::<Card>() {
            Ok(card) => {
                println!();
                println!("You played a {}!", card.display_name());
                println!("{:?}", uno.play_card(card));
                println!();
            },
            Err(cmd) if cmd.trim().to_uppercase() == "DRAW" => {
                
            },
            Err(e) => println!("Error parsing card from {}: {}", line, e),
        }
    }

    println!("\nThanks for playing! Good bye.");
}