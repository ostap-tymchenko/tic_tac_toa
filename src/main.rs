// use std::{array, io::BufRead, vec};

// fn main() {
//     let a = vec![vec![0,0,0],vec![0,0,0],vec![0,0,0]];
//     println!()
//     display_board(&a);
// }

// fn display_board (board: &Vec<Vec<i32>>) {
//     for line in board {
//         println!("{line:?}");
//     }
// }

use std::io;
use comfy_table::Table;
use comfy_table::modifiers::UTF8_ROUND_CORNERS;
use comfy_table::presets::UTF8_FULL;

fn main() {

    let player = input("do you want to play as X or O? (X/O)");
    let cpu: char;
    if player == 'X' {
        cpu = 'O';
        println!("CPU is O, O goes first");
    } else if player == 'O' {
        cpu = 'X';
        println!("CPU is X, you go first");
    } else {
        panic!("not apropriate awnser X/O");
    }

    let table = create_empty_table();
    println!("{table}");


}

fn create_empty_table () -> comfy_table::Table {
    let mut tic_tac_toe = Table::new();
    tic_tac_toe.load_preset(UTF8_FULL)
               .apply_modifier(UTF8_ROUND_CORNERS)
               .set_header(vec![" ","a", "b", "c"])
               .add_row(vec!["1"," "," "," ",])
               .add_row(vec!["2"," "," "," ",])
               .add_row(vec!["3"," "," "," ",]);
    return tic_tac_toe;
}

fn input(question: &str) -> char {
    let mut typed_input = String::new();
    println!("{question}");
    io::stdin()
        .read_line(&mut typed_input)
        .expect("Failed to read input");

    typed_input.chars().next().expect("inapropriate awnser: to short").to_ascii_uppercase()
}
