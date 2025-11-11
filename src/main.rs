fn computer_output () -> u32 {
    let n: u32 = 1;
    n
}

fn game (computer: u32, person: u32) -> u32 {
    let mut math: i32 = 0;
    let mut outcome: u32 = 3;
    math = computer as i32 - person as i32;
    if math == 0 {
        return 1;
    }
    else if math == 2 || if math == -1 {
        return 0;
    }
    else if math == -2 || if math == 1 {
        return 2;
    }
    else {
        return 3;
    }
}



fn main() {
    let mut outcome: u32 = 3;
    let mut n: u32 = 1;
    println!("Let's play rock, paper, scissors!");
    println!("Enter your choice and press enter. (rock -> 1, paper -> 2, scissors -> 3)");
    




    

    if outcome == 0 {
        println!("Computer wins!");
    }
    else if outcome == 1 {
        println!("Draw!");
    }
    else if outcome == 2 {
        println!("Player wins!");
    }
    else {
        println!("Enter a valid number.");
    }
}
