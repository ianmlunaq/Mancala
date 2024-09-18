use std::{io, usize};

fn new_mancala_board_indexed() -> [u8; 14] {
    let mancala_board = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13];
    mancala_board
}

fn new_mancala_board() -> [u8; 14] {
    let mancala_board = [4, 4, 4, 4, 4, 4, 0, 4, 4, 4, 4, 4, 4, 0];
    mancala_board
}

fn sow(mancala_board: &mut [u8; 14], pit_index: usize) -> bool {
    match pit_index {
        0..6 => {
            let pit_qty: usize = (*mancala_board.get(pit_index).unwrap()).into();
            let last_sow = pit_index + pit_qty;
            println!("{last_sow}");
            mancala_board[pit_index] = 0;

            for i in 1..=pit_qty {
                if pit_index + i > 13 {
                    let i = 2;
                }
                mancala_board[pit_index + i] += 1;
            }

            if last_sow == 6 {
                true
            } else {
                false
            }
        },
        6.. => {
            println!("That's not a valid option!");
            false
        },
    }
}

fn print_mancala_board(mancala_board: [u8; 14]) {
    let mut iterator = mancala_board.iter();

    //Prints top board edge and part of left store
    print!("┌{0:─<4}┬{0:─<29}┬{0:─<4}┐\n│{1:<4}│", "─", " ");

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
}

fn main() {
    

    let mut mancala_board = new_mancala_board();

    loop {
        print_mancala_board(mancala_board);
        let mut pit_index = String::new();

        println!("Choose a pit!");
        io::stdin()
            .read_line(&mut pit_index)
            .expect("Failure");

        match pit_index.trim().parse::<usize>() {
            Ok(pit_index) => sow(&mut mancala_board, pit_index),
            Err(_) => {
                println!("That's not a number!");
                false
            },
        };

    }

}
