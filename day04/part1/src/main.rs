// To use:  `cargo run ../input.txt`
/*

*/


// for cmd-line args
use std::env;

// STRINGS!
use std::string::String;

// files
use std::fs::File;
use std::io::{prelude::*, BufReader}; // for reading line-by-line

use std::collections::VecDeque;


fn main() -> std::io::Result<()> {
    // get file name from input args
    let args: Vec<String> = env::args().collect();
    let fpath = &args[1];

    // open file and set up buff reader
    // 8KB buffer size by default.
    // Or init with `with_capacity(num_bytes, file)` instead of `new()`
    let file = File::open(fpath)?;
    let reader = BufReader::new(file);  
    let line_reader = reader.lines().map(|l| l.unwrap()); // get iterator of lines


    // useful vars
    let mut sum: u32 = 0;


    for line in line_reader {
        let line = line.clone();

        // fun little example, using closures and .retain()

        // separate left and right sides
        // instead of iterating by chars, use split() more liberally.
        let left_right: Vec<&str> = line.split('|').collect();
        let mut winners: VecDeque<&str>;
        let mut right: VecDeque<&str>;
        // println!("{:?}", left_right);


        // collect the data from left and right sides
        winners = left_right[0].split(|c: char| !c.is_digit(10)).collect(); 
        winners.retain(|&stringy| stringy.chars().count() > 0);
        let game_num = winners.pop_front().unwrap();

        right = left_right[1].split(|c: char| !c.is_digit(10)).collect(); 
        right.retain(|&stringy| stringy.chars().count() > 0);

        // println!("{:?} | {:?}", winners, right);

        let mut score_plus = 0;

        // match right side against winners
        for num in &right {
            for num2 in &winners {
                if num == num2 {
                    if score_plus == 0 {
                        score_plus = 1;
                    }
                    else {
                        score_plus *= 2;
                    }

                    break;
                }
            }
        }
        println!("Game {} with a score of {}", game_num, score_plus);
        sum += score_plus;
    }


    // print result
    println!("\n\nThe result is (or should be?):  {}", sum);
    Ok(())
}
