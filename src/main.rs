use rand::Rng;


fn computer_output () -> u32 {
    let mut rng = rand::thread_rng();
    let n: u32 = rng.gen_range(1..=3);
    n
}

fn game (computer: u32, person: u32) -> u32 {
    let mut math: i32 = 0;
    math = computer as i32 - person as i32;
    match math {
        0 => return 1,
        1 | -2 => return 0,
        2 | -1 => return 2,
        _ => return 3
    }
}



fn main() {
    println!("Let's play rock, paper, scissors!");
    println!("Enter your choice and press enter. (rock -> 1, paper -> 2, scissors -> 3)");
    
    let person: u32 = 2;    //paper

    println!("You played {}.", person);
    println!("Computer played {}.", computer_output());
    println!("Game output is {}.", game(computer_output(), person));

    if game(computer_output(), person) == 0 {
        println!("Computer wins!");
    }
    else if game(computer_output(), person) == 1 {
        println!("Draw!");
    }
    else if game(computer_output(), person) == 2 {
        println!("Player wins!");
    }
    else {
        println!("Enter a valid number.");
    }

}
