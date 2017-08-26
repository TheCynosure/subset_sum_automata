extern crate clap;
extern crate rand;
use clap::{Arg, App};
use rand::distributions::{IndependentSample, Range};
use std::process::Command;

mod subset_sum;

struct BoardProps {
    starting_val_low: i32,
    starting_val_high: i32,
    board_width: usize,
    board_height: usize,
}

impl BoardProps {
    fn new() -> Self {
        BoardProps {
            starting_val_low: 0,
            starting_val_high: 0,
            board_width: 0,
            board_height: 0,     
        }
    }
}

fn init_board(vec: &mut Vec<Vec<i32>>, bp: &BoardProps) {
    let mut rng = rand::thread_rng();
    let random_range = Range::new(bp.starting_val_low, bp.starting_val_high);
    for h in 0..bp.board_height {
        vec.push(Vec::new());
        for _ in 0..bp.board_width {
            vec[h].push(random_range.ind_sample(&mut rng));   
        }
    }
}

fn draw_board(vec: &mut Vec<Vec<i32>>, bp: &BoardProps) {
    let color_step: i32 = (bp.starting_val_high - bp.starting_val_low) / 4;
    for (_, val) in vec.iter().enumerate() {
        for (_, inner_val) in val.iter().enumerate() {
            let end_color_escape = "\x1B[39;49m";
            let start_color_escape = if *inner_val < (bp.starting_val_low + color_step) { 
                "\x1B[30m"
            } else if *inner_val >= (bp.starting_val_low + color_step) && *inner_val < (bp.starting_val_low + 2 * color_step) { 
                "\x1B[34m"
            } else if *inner_val >= (bp.starting_val_low + 2 * color_step) && *inner_val < (bp.starting_val_low + 3 * color_step) {
                "\x1B[36m"
            } else {
                "\x1B[37m" 
            };
            print!("{} \u{25A0}{}", start_color_escape, end_color_escape);
        }
        println!();
    }
}

fn optional_push(vec: &Vec<Vec<i32>>, temp_vec: &mut Vec<i32>, x: i32, y: i32) {
    //Pulls a value out of the first vec
    //  Checks if it is valid, then...
    //Pulls a value out of the next vec
    //  Checks if it is valid, then...
    //Pushes that element onto the temporary vector.
    vec.get(x as usize).map(|u_vec| u_vec.get(y as usize).map(|val| temp_vec.push(*val)));
}

fn check_neighbors(vec: &mut Vec<Vec<i32>>, args: &Vec<&str>) {
    let mut new_board: Vec<Vec<i32>> = Vec::new();
    for (h_index, h_val) in vec.iter().enumerate() {
        let h_index = h_index as i32;
        new_board.push(Vec::new());
        for (w_index, _) in h_val.iter().enumerate() {
            let mut temp_vec: Vec<i32> = Vec::new();
            let w_index = w_index as i32;
            optional_push(&vec, &mut temp_vec, h_index - 1, w_index - 1);
            optional_push(&vec, &mut temp_vec, h_index - 1, w_index);
            optional_push(&vec, &mut temp_vec, h_index - 1, w_index + 1);
            optional_push(&vec, &mut temp_vec, h_index, w_index - 1);
            optional_push(&vec, &mut temp_vec, h_index, w_index + 1);
            optional_push(&vec, &mut temp_vec, h_index + 1, w_index - 1);
            optional_push(&vec, &mut temp_vec, h_index + 1, w_index);
            optional_push(&vec, &mut temp_vec, h_index + 1, w_index + 1);
            if subset_sum::check_no_empty_subset(temp_vec, args[0].parse().unwrap()) {
                new_board[h_index as usize].push(vec[h_index as usize][w_index as usize]+ (args[1].parse::<i32>().unwrap()));     
            } else {
                new_board[h_index as usize].push(vec[h_index as usize][w_index as usize] + (args[2].parse::<i32>().unwrap()));
            }
        }
    }

    for (h_index, h_val) in new_board.iter().enumerate() {
        for (w_index, w_val) in h_val.iter().enumerate() {
            vec[h_index][w_index] = *w_val;
        }
    }
}

fn main() {
    let matches = App::new("Subset Sum Automata")
        .version("1.0")
        .author("Jack B. <jack.github@gmail.com>")
        .about("Solves a DailyProgrammer question")
        .arg(Arg::with_name("INPUT")
            .help("Target/Increment/Decrement In that order")
            .required(true)
            .index(1))
        .arg(Arg::with_name("width")
            .short("w")
            .long("width")
            .value_name("width_val")
            .help("Set the width in columns.")
            .takes_value(true))
        .arg(Arg::with_name("height")
            .short("h")
            .long("height")
            .value_name("height_val")
            .help("Set the height in rows.")
            .takes_value(true))
        .arg(Arg::with_name("min")
            .short("i")
            .long("min")
            .value_name("min_val")
            .help("Sets the lower limit of the random target value range.")
            .takes_value(true))
        .arg(Arg::with_name("max")
            .short("a")
            .long("max")
            .value_name("max_val")
            .help("Sets the upper limit of the random target value range.")
            .takes_value(true))
        .get_matches();
    
    let mut bp = BoardProps::new();
    
    bp.board_width = match matches.value_of("width") { Some(x) => x.parse().unwrap(), None => 10, };
    bp.board_height = match matches.value_of("height") { Some(x) => x.parse().unwrap(), None => 10, };

    bp.starting_val_low = match matches.value_of("min") { Some(x) => x.parse().unwrap(), None => 0, };
    bp.starting_val_high = match matches.value_of("max") { Some(x) => x.parse().unwrap(), None => 100, };
        
    let args: Vec<&str> = matches.value_of("INPUT").unwrap().split('/').collect();
    
    let mut vec: Vec<Vec<i32>> = Vec::new();
    init_board(&mut vec, &bp);
    Command::new("clear").spawn().expect("Failed to clear screen");
    loop {
        draw_board(&mut vec, &bp);
        check_neighbors(&mut vec, &args);
        println!("\x1B[{}A\x1B[{}D", bp.board_height + 1, bp.board_width * 2);
    }
}