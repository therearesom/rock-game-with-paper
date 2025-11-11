fn computer_output (n: u32) -> u32 {
    
   n 
}




fn main() {
    let mut outcome: u32 = 3;
    println!("Let's play rock, paper, scissors!");
    println!("Enter your choice. (rock -> 1, paper -> 2, scissors -> 3)");
    
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
