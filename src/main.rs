extern crate rand;

use std::io;
use rand::Rng;

fn main() {
    println!("Rock Paper Scissors");
    println!("1 - Rock, 2 - Paper, 3 - Scissors");

    let mut p_points = 0;
    let mut c_points = 0;

    loop {
        println!("Please input your throw.");

        let mut p_throw = String::new();

        io::stdin().read_line(&mut p_throw)
            .ok()
            .expect("Failed to read line");

        let p_throw: u32 = match p_throw.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        let c_throw = rand::thread_rng().gen_range(1, 3);

        println!("Plyr threw: {}", p_throw);
        println!("Comp threw: {}", c_throw);

        if p_throw == c_throw {
            println!("Player and Computer throw the same thing; TIE!");
        }
        else if (p_throw == 1 && c_throw == 2) {
            println!("Player's rock is covered by Computer's paper; LOSE!");
            c_points += 1;
        }
        else if (p_throw == 1 && c_throw == 3) {
            println!("Player rock bashes Computer's scissors: WIN!");
            p_points += 1;
        }
        else if (p_throw == 2 && c_throw == 1) {
            println!("Player's paper covers Computer's rock; WIN!");
            p_points += 1;
        }
        else if (p_throw == 2 && c_throw == 3) {
            println!("Player's paper is cut by Computer's scissors; LOSE!");
            c_points += 1;
        }
        else if (p_throw == 3 && c_throw == 1) {
            println!("Player's scissors are bashed by Computer's rock; LOSE!");
            c_points += 1;
        }
        else if (p_throw == 3 && c_throw == 2) {
            println!("Player's scissors cut through Computer's paper; WIN!");
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

    println!("Thank you for playing RPS!");
}
