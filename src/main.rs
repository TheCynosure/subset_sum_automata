extern crate rand;
use rand::distributions::{IndependentSample, Range};
use std::env;
use std::process::Command;
use std::{thread, time};

mod subset_sum;

const STARTING_VAL_LOW: i32 = 0;
const STARTING_VAL_HIGH: i32 = 50;
const BOARD_WIDTH: usize = 100;
const BOARD_HEIGHT: usize = 75;

fn init_board(vec: &mut Vec<Vec<i32>>) {
    let mut rng = rand::thread_rng();
    let random_range = Range::new(STARTING_VAL_LOW, STARTING_VAL_HIGH);
    for h in 0..BOARD_HEIGHT {
        vec.push(Vec::new());
        for _ in 0..BOARD_WIDTH {
            vec[h].push(random_range.ind_sample(&mut rng));   
        }
    }
}

fn draw_board(vec: &mut Vec<Vec<i32>>) {
    const COLOR_STEP: i32 = (STARTING_VAL_HIGH - STARTING_VAL_LOW) / 4;
    for (_, val) in vec.iter().enumerate() {
        for (_, inner_val) in val.iter().enumerate() {
            let end_color_escape = "\x1B[39;49m";
            let start_color_escape = if *inner_val < (STARTING_VAL_LOW + COLOR_STEP) { 
                "\x1B[30m"
            } else if *inner_val >= (STARTING_VAL_LOW + COLOR_STEP) && *inner_val < (STARTING_VAL_LOW + 2 * COLOR_STEP) { 
                "\x1B[34m"
            } else if *inner_val >= (STARTING_VAL_LOW + 2 * COLOR_STEP) && *inner_val < (STARTING_VAL_LOW + 3 * COLOR_STEP) {
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
    //let args: Vec<String> = env::args().collect();
    let args = vec!["5", "1", "-1"];    
    
    let mut vec: Vec<Vec<i32>> = Vec::new();
    init_board(&mut vec);
    Command::new("clear").spawn().expect("Failed to clear screen");
    loop {
        draw_board(&mut vec);
        check_neighbors(&mut vec, &args);
        println!("\x1B[{}A\x1B[{}D", BOARD_HEIGHT + 1, BOARD_WIDTH * 2);
    }
}