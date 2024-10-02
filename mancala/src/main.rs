use std::{io, usize, unreachable};
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
        _ => unreachable!("How did we get here?"),
    }
}

fn new_mancala_board_custom() -> [u8; 14] {
    let mancala_board = [4, 4, 4, 4, 4, 1, 0, 0, 0, 0, 0, 0, 14, 0];
    mancala_board
}

fn new_mancala_board() -> [u8; 14] {
    let mancala_board = [4, 4, 4, 4, 4, 4, 0, 4, 4, 4, 4, 4, 4, 0];
    mancala_board
}

// todo: add capture mechanic
// cp_sow returns false to indicate CP gets another turn and returns true to indicate their turn is over
fn cp_sow(mancala_board: &mut [u8; 14]) -> bool {

    // This if statement inside a loop ensures an empty pit is not selected
    let mut pit_index;
    loop { // todo: fix infiniloop
        pit_index = rand::thread_rng().gen_range(7..=12);
        if mancala_board[pit_index] != 0 { break; }
    }
    
    print!("{} chooses {}!", "CP".blue(), number_to_keybind_letter(pit_index));
    let pit_qty: usize = (*mancala_board.get(pit_index).unwrap()).into();
    let mut last_sow:usize = 0;
    mancala_board[pit_index] = 0;

    for i in 1..=pit_qty {
        let mut current_pit = pit_index + i;

        // This if statements ensure the sowing does not go
        // out of bounds of the vector and wraps around
        if current_pit > 13 {
            current_pit -= 14;

            // This if statement blocks sowing into opponent's store
            if current_pit > 5 {
                current_pit += 1;
            }
        }

        println!("Current: {current_pit}");

        mancala_board[current_pit] += 1;
        last_sow = current_pit;
    }

    println!("Last sow: {last_sow}");

    if last_sow == 13 {
        false
    } else {
        true
    }
    
}

// todo: add capture mechanic
// sow returns false to indicate player gets another turn and returns true to indicate their turn is over
fn sow(mancala_board: &mut [u8; 14], pit_index: usize) -> bool {
    let pit_qty: usize = (*mancala_board
        .get(pit_index)
        .unwrap_or(&100))
        .into();

    // This match statement ensures the player's pit selection is valid (i.e. index 0-5 and not empty)
    match pit_index {
        0..6 if pit_qty != 0 => {
            mancala_board[pit_index] = 0;

            // last_sow is updated after every iteration of the for loop to ensure its value is that
            // of the index of the final sow. It's then checked to see if player gets a bonus turn.
            let mut last_sow: usize = 0;

            // This for loop adds 1 to each pit proceeding pit_index
            for i in 1..=pit_qty {
                let mut current_pit = pit_index + i;

                // This if statement ensures the sowing does not go out of bounds of the vector
                // and wraps around to the beginning without sowing into the opponent's store
                if current_pit > 12 {
                    current_pit -= 13;
                }

                mancala_board[current_pit] += 1;

                last_sow = current_pit;
            }

            // This if statement checks last_sow to see if player gets a bonus turn
            if last_sow == 6 {
                print!(" Go again! ");
                false
            } else {
                true
            }
        },
        _ => {
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

// todo: implement endgame bahavior
fn main() {
    let mut mancala_board = new_mancala_board_custom();

    loop {
        print_mancala_board(mancala_board);

        // Human player loop
        loop {
            let input = get_user_input("Choose a letter!");
            let pit_letter = input.get(0..1).unwrap().parse::<char>().unwrap().to_ascii_uppercase();
            print!("\nYou chose \"{}\"!", pit_letter.to_string().trim().bright_yellow());

            let turn_over = sow(&mut mancala_board, keybind_letter_to_number(pit_letter));
            print_mancala_board(mancala_board);

            if turn_over { break; }
        }

        print!("{} turn next! ", "CP's".blue());
        get_user_input("Press [Enter] to continue...");

        // CP loop
        loop {
            if cp_sow(&mut mancala_board) { break;}
            print_mancala_board(mancala_board);
            print!("{} goes again! ", "CP".blue());
            get_user_input("CP Press [Enter] to continue...");
        }
    }
}
