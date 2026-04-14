use std::io::{self, Write};

use filler::parser::{read_player, read_turn};
use filler::placement::find_valid_placements;
use filler::strategy::choose_best;

fn main() {
    let stdin = io::stdin();
    let mut reader = stdin.lock();

    let player = read_player(&mut reader).expect("Failed to read player number");

    eprintln!("I am {:?}", player);

    loop {
        let turn_result = read_turn(&mut reader);

        match turn_result {
            Ok((grid, piece)) => {
                // Find all valid placements
                let valid = find_valid_placements(&grid, &piece, &player);

                if valid.is_empty() {
                    println!("0 0");
                } else {
                    let best = choose_best(&valid, &piece, &grid, &player);
                    println!("{} {}", best.x, best.y);
                }

                io::stdout().flush().unwrap();
            }
            Err(_) => {
                break;
            }
        }
    }
}
