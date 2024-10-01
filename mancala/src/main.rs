use std::{io, usize};
use colored::Colorize;
use rand::Rng;
//use term::Terminal;

fn keybind_letter_to_number(keybind_letter: char) -> usize {
    match keybind_letter {
        'A' => 0,
        'S' => 1,
        'D' => 2,
        'F' => 3,
        'J' => 4,
        'K' => 5,
        _ => 100,
    }
}

fn number_to_keybind_letter(number: usize) -> char {
    match number {
        0 | 12 => 'A',
        1 | 11 => 'S',
        2 | 10 => 'D',
        3 | 9 => 'F',
        4 | 8 => 'J',
        5 | 7 => 'K',
        _ => '?',
    }
}


/*fn new_mancala_board_indexed() -> [u8; 14] {
    let mancala_board = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13];
    mancala_board
}*/

fn new_mancala_board() -> [u8; 14] {
    let mancala_board = [4, 4, 4, 4, 4, 4, 0, 4, 4, 4, 4, 4, 4, 0];
    mancala_board
}

// todo: fix invalid 0-pit selections
fn cp_sow(mancala_board: &mut [u8; 14]) -> bool {
    let pit_index = rand::thread_rng().gen_range(7..=12);
    print!("{} chooses {}!", "CP".blue(), number_to_keybind_letter(pit_index));
    let pit_qty: usize = (*mancala_board.get(pit_index).unwrap()).into();
    let last_sow = pit_index + pit_qty;
    mancala_board[pit_index] = 0;

    for i in 1..=pit_qty {
        let mut current_pit = pit_index + i;

        //These 2 if statements ensure the sowing does not go out of bounds of 
        // the vector and wraps around without sowing into the opponent's store
        if current_pit > 13 {
            current_pit -= 14;
        }
        if current_pit == 6 {
            continue;
        }

        mancala_board[current_pit] += 1;
    }

    if last_sow == 13 {
        true
    } else {
        false
    }
    
}

// todo: fix invalid 0-pit selections
// todo: return player redo elegibility
fn sow(mancala_board: &mut [u8; 14], pit_index: usize) -> bool {
    match pit_index {
        0..6 => {
            let pit_qty: usize = (*mancala_board.get(pit_index).unwrap()).into();
            //let last_sow = pit_index + pit_qty;
            mancala_board[pit_index] = 0;

            for i in 1..=pit_qty {
                let mut current_pit = pit_index + i;

                //This if statement ensures the sowing does not go out of bounds of 
                // the vector and wraps around without sowing into the opponent's store
                if current_pit > 12 {
                    current_pit -= 13;
                }

                mancala_board[current_pit] += 1;
            }

            true

            /* if last_sow == 6 {
                true
            } else {
                false
            } */
        },
        6.. => {
            println!("{}", " That's not a valid option!".bright_red());
            false
        },
    }
}

fn print_mancala_board(mancala_board: [u8; 14]) {
    let mut iterator = mancala_board.iter();

    //Prints top board edge and part of left store
    print!("\n┌{0:─<4}┬{0:─<29}┬{0:─<4}┐\n│{1:<4}│", "─", " ");

    //Consumes the last iterator and begins printing 2nd player's pits in reverse order (indexes 7-12) and top of right store
    iterator.next_back();
    for _i in 0..6 {
        print!("{:^4}│", iterator.next_back().unwrap());
    }
    println!("{:^4}│", " ");

    //Prints both stores and middle line
    print!("│{2:<4}├{:─<29}┤{3:>4}│\n│{1:<4}│", "─", " ", mancala_board.get(13).unwrap(), mancala_board.get(6).unwrap());

    //Prints 1st player's pits (indexes 0-5) and part of right store
    for _i in 0..6 {
        print!("{:^4}│", iterator.next().unwrap());
    }
    println!("{:^4}│", " ");

    //Prints bottom edge
    println!("└{0:─<4}┴{0:─<29}┴{0:─<4}┘", "─");

    //Prints keybinds
    println!("{0:6}{1:^4} {2:^4} {3:^4} {4:^4} {5:^4} {6:^4}\n", " ",
    "A".bright_yellow().bold(), "S".bright_yellow().bold(), "D".bright_yellow().bold(),
    "F".bright_yellow().bold(), "J".bright_yellow().bold(), "K".bright_yellow().bold());

}

fn get_user_input(prompt: &str) -> String {
    println!("{prompt}");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failure");
    input
}

fn main() {
    let mut mancala_board = new_mancala_board();

    loop {
        print_mancala_board(mancala_board);

        loop {
            let input = get_user_input("Choose a letter!");

            let pit_letter = input.get(0..1).unwrap().parse::<char>().unwrap().to_ascii_uppercase();
            print!("\nYou chose \"{}\"!", pit_letter.to_string().trim().bright_yellow());

            if sow(&mut mancala_board, keybind_letter_to_number(pit_letter)) { break; }
        }

        print_mancala_board(mancala_board);

        get_user_input("CP's turn next! Press [Enter] to continue...");

        loop {
            if !cp_sow(&mut mancala_board) { break;}
            print_mancala_board(mancala_board);
            get_user_input("CP goes again! Press [Enter] to continue...");
        }
    }
}
