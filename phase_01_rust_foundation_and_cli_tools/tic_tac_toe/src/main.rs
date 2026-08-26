use colored::Colorize;
use std::io;
fn main() {
    let mut ttt = [["1", "2", "3"], ["4", "5", "6"], ["7", "8", "9"]];
    let mut choice = String::new();
    let mut step = 0;
    let mut current_player = "X";
    loop {
        print_tab(&ttt);
        println!(
            "{}",
            format!("Player {} enter the position[1-9]", current_player)
                .red()
                .bold()
        );

        choice.clear();
        io::stdin().read_line(&mut choice).unwrap();
        let choice: usize = match choice.trim().parse::<usize>() {
            Ok(num) if num >= 1 && num <= 9 => num,
            _ => {
                println!("Please enter a Digit[1-9]");
                continue;
            }
        };
        let row = (choice - 1) / 3;
        let col = (choice - 1) % 3;
        if ttt[row][col] == "X" || ttt[row][col] == "O" {
            println!("{}", format!("Cell is already filled").red().bold());
            continue;
        } else {
            ttt[row][col] = current_player;
            step += 1;
        }
        if ttt[0][0] == current_player && ttt[0][1] == current_player && ttt[0][2] == current_player
        {
            print_pro(&ttt, current_player, (0, 0), (0, 1), (0, 2));
            break;
        }
        if ttt[1][0] == current_player && ttt[1][1] == current_player && ttt[1][2] == current_player
        {
            print_pro(&ttt, current_player, (1, 0), (1, 1), (1, 2));
            break;
        }
        if ttt[2][0] == current_player && ttt[2][1] == current_player && ttt[2][2] == current_player
        {
            print_pro(&ttt, current_player, (2, 0), (2, 1), (2, 2));
            break;
        }
        if ttt[0][0] == current_player && ttt[1][1] == current_player && ttt[2][2] == current_player
        {
            print_pro(&ttt, current_player, (0, 0), (1, 1), (2, 2));
            break;
        }
        if ttt[0][0] == current_player && ttt[1][0] == current_player && ttt[2][0] == current_player
        {
            print_pro(&ttt, current_player, (0, 0), (1, 0), (2, 0));
            break;
        }
        if ttt[0][1] == current_player && ttt[1][1] == current_player && ttt[2][1] == current_player
        {
            print_pro(&ttt, current_player, (0, 1), (1, 1), (2, 1));
            break;
        }
        if ttt[0][2] == current_player && ttt[1][2] == current_player && ttt[2][2] == current_player
        {
            print_pro(&ttt, current_player, (0, 2), (1, 2), (2, 2));
            break;
        }
        if ttt[0][2] == current_player && ttt[1][1] == current_player && ttt[2][0] == current_player
        {
            print_pro(&ttt, current_player, (0, 2), (1, 1), (2, 0));
            break;
        }
        if step == 9 {
            println!("{}", format!("\nGame is draw!\n\n\n\n\n\n").yellow().bold());
            break;
        }
        current_player = if current_player == "X" { "O" } else { "X" };
    }
}

fn print_tab(ttt: &[[&str; 3]; 3]) {
    println!("\n\n");
    println!("|-----|-----|-----|");
    for r in 0..3 {
        for c in 0..3 {
            if ttt[r][c] == "X" {
                print!("|  {}  ", ttt[r][c].red());
            } else if ttt[r][c] == "O" {
                print!("|  {}  ", ttt[r][c].green());
            } else {
                print!("|  {}  ", ttt[r][c]);
            }
        }
        println!("|");

        if r < 3 {
            println!("|-----|-----|-----|");
        }
    }
    println!("\n\n");
}

fn print_pro(
    ttt: &[[&str; 3]; 3],
    current_player: &str,
    p1: (usize, usize),
    p2: (usize, usize),
    p3: (usize, usize),
) {
    if current_player == "X" {
        println!("\nPlayer {} you won the game\n\n\n\n", "X".red());
    }
    if current_player == "O" {
        println!("\nPlayer {} you won the game\n\n\n\n", "O".green());
    }
    println!("|-----|-----|-----|");
    for r in 0..3 {
        print!("| ");
        for c in 0..3 {
            if (r, c) == p1 || (r, c) == p2 || (r, c) == p3 {
                if current_player == "X" {
                    print!(" {} ", ttt[r][c].red().bold());
                }
                if current_player == "O" {
                    print!(" {} ", ttt[r][c].green().bold());
                }
            } else {
                print!(" {} ", ttt[r][c].bright_black());
            }
            if c < 2 {
                print!(" | ");
            }
        }
        println!(" |");
        if r < 2 {
            println!("|-----|-----|-----|");
        }
    }
    println!("|-----|-----|-----|");
    println!("\n\n\n\n\n\n\n\n\n\n");
}
