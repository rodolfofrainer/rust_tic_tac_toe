use std::io;
fn main() {
    let mut values: [&str; 9] = [" "; 9];

    println!("Welcome to Tic Tac Toe!");
    print_board(&values);
    loop {

        println!("Please enter a position to place your X (1-9):");
        println!("----------------------------------------------");

        let mut position = String::new();

        io::stdin()
            .read_line(&mut position)
            .expect("Failed to read line");

        let mut position: usize = match position.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        if position <1 || position >9{
            println!("Please enter a valid position (1-9)");
            continue;
        }

        position -=1;
        
        if values[position] == "X" || values[position] == "O" {
            println!("That position is already taken!");
            continue;
        }
        
        values[position] = "X";

        let mut next_position = (position + 1) % 9;
        let mut is_tie = true;
        for _ in 0..9{
            if values[next_position] == " " {
                is_tie = false;
                break;
            }
            next_position = (next_position + 1) % 9;
        }

        if is_tie {
            println!("It's a tie!");
            break;
        }

        while values[next_position] == "X" || values[next_position] == "O" {
            next_position = (next_position + 1) % 9;
        }
        values[next_position] = "O";
        if let Some(winner) = check_winner(&values) {
            print_board(&values);
            println!("{} wins!", winner);
            break;
        }
        
        print_board(&values);
        println!("");
    }
}

fn print_board(&values: &[&str; 9]) {
    println!("{} | {} | {}", values[0], values[1], values[2]);
    println!("---------");
    println!("{} | {} | {}", values[3], values[4], values[5]);
    println!("---------");
    println!("{} | {} | {}", values[6], values[7], values[8]);
}

fn check_winner<'a>(values: &'a[&'a str; 9]) -> Option<&'a str> {
    if values[0] == values[1] && values[1] == values[2] && values[0] != " " ||
       values[0] == values[3] && values[3] == values[6] && values[0] != " " ||
       values[0] == values[4] && values[4] == values[8] && values[0] != " "
    {
        Some(values[0])
    } else if values[3] == values[4] && values[4] == values[5] && values[3] != " " ||
              values[1] == values[4] && values[4] == values[7] && values[1] != " " ||
              values[2] == values[4] && values[4] == values[6] && values[2] != " "
    {
        Some(values[4])
    } else if values[6] == values[7] && values[7] == values[8] && values[6] != " " ||
              values[2] == values[5] && values[5] == values[8] && values[2] != " "
    {
        Some(values[8])
    } else {
        None
    }
}
