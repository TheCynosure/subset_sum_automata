/*
 * The Problem is from here: https://www.reddit.com/r/dailyprogrammer/comments/68oda5/20170501_challenge_313_easy_subset_sum/
 * The point of the problem is to see in a sorted list if any two integers would equal 0.
 * This seems to be very simple as we are just looking for the inverse of the largest number.
 * We should be able to this in O(n)
 */

use std::io;
use std::io::BufRead;

#[allow(dead_code)]
fn check_subset(vec: Vec<i32>) -> bool {
    let mut less_index = 0;
    let mut greater_index = vec.len() - 1;
    if vec[less_index] > 0 {
        //There is no way that a list of all positive numbers
        //can ever be less than zero.
        return false;     
    }
    while less_index < greater_index {
        //Check if we have equal values.
        //Or if either of the values are zero.
        if vec[less_index] == -vec[greater_index] || vec[less_index] == 0 || vec[greater_index] == 0 {
            return true;     
        }
        less_index += 1;
        greater_index -= 1;
    }
    //If there was never any cancelling values or zero found,
    //then there is not 1-2 length subset that sums to zero.
    false
}

fn check_no_empty_subset(vec: Vec<i32>) -> bool {
    let mut subset_mask = 1;
    let subset_limit =  2_i32.pow(vec.len() as u32);
    while subset_mask < subset_limit {
        let mut mask_copy = subset_mask;
        let mut sum: i32 = 0;
        for (i, _) in vec.iter().enumerate() {
            if (mask_copy & 1) == 1 {
                sum += vec[i];
            }
            mask_copy = mask_copy >> 1;    
        }

        if sum == 0 {
            return true;     
        }
        subset_mask += 1;
    }
    false
}

fn main() {
    //Simple input, no error checking here.
    //All numbers must be seperated by commas.
    //Square brackets will be removed.
    loop {
        let stdin = io::stdin();
        let mut array_input = String::new();
        println!("Please enter the array: ");

        let mut handle = stdin.lock();
        
        handle.read_line(&mut array_input).expect("Not a correct string");

        if array_input == "quit" {
            break;     
        }

        let array_input = array_input.replace("[", "");
        let array_input = array_input.replace("]", "");
        let array_input = array_input.replace(" ", "");
        let array_input = array_input.replace("\n", "");
        
        if array_input == "" {
            println!("false");
            continue;     
        }
        
        let array: Vec<i32> = array_input.split(",").map(|s| s.parse().unwrap() ).collect();
        println!("{}", check_no_empty_subset(array));
    }
}
