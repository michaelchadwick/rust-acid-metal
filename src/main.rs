extern crate rand;

use std::io;
use rand::Rng;

fn main() {
    println!("Rust Acid Metal");
    // rock(rust), paper(acid), scissors(metal)
    println!("1 - Rust, 2 - Acid, 3 - Metal");
    println!("Rust beats metal, acid beats rust, and metal beats acid");
    println!("");
    println!("First to 5 points wins the battle");
    println!("");

    let mut p_points = 0;
    let mut c_points = 0;

    loop {
        println!("Please input your throw.");

        // get player's throw
        let mut p_throw = String::new();

        io::stdin().read_line(&mut p_throw)
            .ok()
            .expect("Failed to read line");

        let p_throw: u32 = match p_throw.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        // throw is out of range, so ignore
        if p_throw < 1 || p_throw > 3 {
            continue;
        }

        // get computer's throw
        let c_throw = rand::thread_rng().gen_range(1, 3);

        println!("Plyr threw: {}", p_throw);
        println!("Comp threw: {}", c_throw);

        // parse moves
        if p_throw == c_throw {
            println!("Player and Computer throw the same thing; TIE!");
        }
        else if p_throw == 1 && c_throw == 2 {
            println!("Player's rust is dissolved by Computer's acid; LOSE!");
            c_points += 1;
        }
        else if p_throw == 1 && c_throw == 3 {
            println!("Player rust takes over Computer's metal: WIN!");
            p_points += 1;
        }
        else if p_throw == 2 && c_throw == 1 {
            println!("Player's acid dissolves Computer's acid; WIN!");
            p_points += 1;
        }
        else if p_throw == 2 && c_throw == 3 {
            println!("Player's acid is beaten by Computer's metal; LOSE!");
            c_points += 1;
        }
        else if p_throw == 3 && c_throw == 1 {
            println!("Player's metal is taken over by Computer's rust; LOSE!");
            c_points += 1;
        }
        else if p_throw == 3 && c_throw == 2 {
            println!("Player's metal beats Computer's acid; WIN!");
            p_points += 1;
        }

        println!("Player {}, Computer {}", p_points, c_points);

        if p_points >= 5 {
            println!("Player wins the whole thing! WOO!");
            break;
        }
        if c_points >= 5 {
            println!("Player loses to a random number generator! :-(");
            break;
        }
    }

    println!("Thank you for playing Rust Acid Metal!");
}

#[test]
fn it_works() {
    assert!(true);
}
