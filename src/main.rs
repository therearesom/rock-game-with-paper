use rand::Rng;
use std::io;

fn computer_output () -> u32 {
    let mut rng = rand::thread_rng();
    let n: u32 = rng.gen_range(1..=3);
    n
}

fn game (computer: u32, person: u32) -> u32 {
    let math: i32 = computer as i32 - person as i32;
    match math {
        0 => return 1,
        1 | -2 => return 0,
        2 | -1 => return 2,
        _ => return 3
    }
}

fn main() {
    println!("Let's play rock, paper, scissors!");
    let mut input = String::new();
    println!("Enter your choice and press enter. (rock -> 1, paper -> 2, scissors -> 3)");
    
    io::stdin().read_line(&mut input).expect("Reading failed");

    let person: u32 = input.trim().parse().expect("Enter a valid number!");
    let computer: u32 = computer_output ();

    println!("You played {}.", person);
    println!("Computer played {}.", computer);
    println!("Game output is {}.", game(computer, person));

    match game(computer, person) {
        0 => println!("Computer wins!"),
        1 => println!("Draw!"),
        2 => println!("Player wins!"),
        _ => println!("Enter a valid number."),
    }
}
