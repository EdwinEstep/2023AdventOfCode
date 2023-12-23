// To use:  `cargo run ../input.txt`
/*
This definitely seems to get a bit harder. However, it might be easy
to keep track of just how many copies of each card there are in some kind of
deque, rather than ... <coding>

Okay, after finishing coding it up, it was just that. Don't need to manipulate
the text, just keep an up-to-date record of the card count for each card #,
and adjust accordingly for each win.
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

    // read in all the existing scratchcards
    let mut file_lines = VecDeque::new();
    let mut card_counts = VecDeque::new();
    let mut initial_cards = 0;
    for line in line_reader {
        file_lines.push_back(line.clone());
        initial_cards += 1;
        card_counts.push_back(1);
    }
    println!("Initially, there are {} cards.", initial_cards);

    // useful vars
    let mut sum: u32 = 0;

    for line in file_lines {
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
        let card_index = winners.pop_front().unwrap().parse::<u32>().unwrap() - 1;

        right = left_right[1].split(|c: char| !c.is_digit(10)).collect(); 
        right.retain(|&stringy| stringy.chars().count() > 0);

        // println!("{:?} | {:?}", winners, right);

        let mut num_won = 0;

        // match right side against winners
        for num in &right {
            for num2 in &winners {
                if num == num2 {
                    num_won += 1;
                    break;
                }
            }
        }
        println!("Card {} won {} times!", card_index+1, num_won);

        // add additional cards, accounting for multipliers due to
        // existing multiples of cards
        if num_won > 0 {
            for index in (card_index+1)..=(card_index+num_won) {
                card_counts[index.try_into().unwrap()] += card_counts[card_index.try_into().unwrap()];
            }
        }

        println!("{:?}", card_counts);
    }

    for num in card_counts {
        sum += num;
    }

    // print result
    println!("\n\nThe result is:  {}", sum);
    Ok(())
}
