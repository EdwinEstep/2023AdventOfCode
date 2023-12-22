// To use:  `cargo run ../input.txt`
/*
At first I thought that I could solve part 2 simply by replacing my
part # search with a gear search, making the problem easier than part1.
However, that didn't allow me to easily discover any other part nums
adjacent to a gear.
One benefit of this method is that it's easy to find how many parts are adjacent to a gear.

A better idea might be to search for more part nums once you've already found the first
part num adjacent to a gear. In this instance, you directly copy part 1 and then ...

On second thought, I'm going to retry the first method. For every gear I find (2 part #s),
I can just iterate through the strings to complete the part #, copying my code from part 1.
Then, if the indices of the part # fall in range to be adjacent to the gear, I add it to
a vec. The vec results will be multiplied together and added to the sum.


Final Solution:
I ended up going with my instinct and reusing the int-parsing from part1. This resulted
in a huge amount of nested logic, but it works!
*/


// for cmd-line args
use std::env;

// STRINGS!
use std::string::String;

// files
use std::fs::File;
use std::io::{prelude::*, BufReader}; // for reading line-by-line

// for circular buffer
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

    // put into a vec of strings
    // include blank data at start and end for easy processing
    let mut file_lines = VecDeque::new();
    for mut line in line_reader {
        line.push('.');
        file_lines.push_back(line.clone());
    }
    let line_length = file_lines[0].chars().count();
    file_lines.push_front(std::iter::repeat(".").take(line_length).collect::<String>());
    file_lines.push_back(std::iter::repeat(".").take(line_length).collect::<String>());


    // line buffer, stores the most recent three lines, so that
    // it's easier to find adjacent symbols.
    let mut line_buf = VecDeque::new();

    // useful vars
    let mut sum: u32 = 0;
    
    // pre-fill the buffer (assumes first line is empty of useful data)
    line_buf.push_back(file_lines[0].to_owned());
    line_buf.push_back(file_lines[1].to_owned());
    file_lines.pop_front();
    file_lines.pop_front();


    for line in file_lines {
        let line = line.clone();

        // // fun little example, using closures and .retain()
        // let mut thingy: Vec<&str> = line_buf[1].split(|c: char| !c.is_digit(10)).collect();
        // thingy.retain(|&stringy| stringy.chars().count() > 0);
        // println!("{:?}", thingy);

        line_buf.push_back(line.clone());

        // per-part# flags
        let mut col;
        let mut ind1;
        let mut ind2;
        
        // iterate through characters until a digit is found
        col = 0;
        for c in line_buf[1].chars() {
            if c == '*' {
                if col > 0 {
                    ind1 = col - 1;
                }
                else {
                    ind1 = 0;
                }

                if !(col < line_length) {
                    ind2 = col;
                }
                else {
                    ind2 = col + 2;
                }

                // ======= FIND THE RECTANGLE HERE ====== //
                let mut rect_string = String::new();
                for line in &line_buf {
                    rect_string.push_str(&line[ind1..ind2]);
                    rect_string.push(',');
                }
                // print!(" | {}", rect_string);
                

                // === GET NUMBER OF PARTS === //
                let mut num_parts = 0;
                let mut part_done = true;
                
                // find distinct parts
                for c in rect_string.chars() {
                    if c.is_digit(10) {
                        // print!("{}", c);
        
                        part_done = false;
                    }
                    else {
                        if !part_done {

                            num_parts += 1;
                        }
                        part_done = true;
                    }
                }
                // println!("\nThis gear is adjacent to {num_parts} parts");


                // === GET COMPLETE PART NUMBERS === //
                let mut part_nums = Vec::new();
                let mut part_str = String::new();
                let mut count = 0;
                ind1 = 0;

                if num_parts == 2 {
                    println!("{:?}", line_buf);

                    for l in &line_buf {
                        for c in l.chars() {
                            if c.is_digit(10) {
                                part_str.push(c);
                                
                                // detect if this is a new part #
                                if part_done {
                                    // check if num is at left-most edge
                                    if count == 0 {
                                        ind1 = count;
                                    }
                                    else {
                                        // if not, get index just left of partnum
                                        ind1 = count - 1;
                                    }

                                }
                                part_done = false;
                            }
                            else {
                                // finished reading part #
                                if !part_done {
                                    ind2 = count;

                                    // check that the part is adjacent to the gear
                                    // (indices within bounds)
                                    if ind1 <= col && col <= ind2 {
                                        part_nums.push(part_str.parse::<u32>().unwrap());
                                        println!("{:?}, count={count}, ind1={ind1}, ind2={ind2}", part_nums);
                                    }

                                    part_str = String::new();
                                    part_done = true;
                                }
                            }

                            count += 1;
                        }

                        count = 0;
                    }

                    sum += part_nums[0] * part_nums[1];
                }
            }

            col += 1;
        }

        // println!("{:?}", line_buf);
        // println!("\n<==>\n");
        line_buf.pop_front();
    }


    // print result
    println!("\n\nThe result is (or should be?):  {}", sum);
    Ok(())
}
