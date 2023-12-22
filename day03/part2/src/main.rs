// To use:  `cargo run ../input.txt`
/*
Notes:
This day is interesting, because you have to remember data from previous lines
of text. 

Part 1 doesn't seem too bad. There are a few parts to the puzzle:
1. Need to keep multiple lines in mem at once and go back to reference them
later.
2. Search through the middle line(1) in the queue [0 _1_ 2], and scan through
iteratively. If a numeric digit character is found, record where. Then
continue iterating until you find a non-numeric character. Record that too.
Now you know the bounds of your number, and, by extension, the bounding box
to search for special characters in.
3. Write a function that 
- takes in a 2D array of characters as well as the
  first and last coordinates of a number in that array.
- returns the string of data adjacent to that (a rectangle), flattened.
- If the number is adjacent to the left or right side of the char array,
  the missing characters are assumed to be '.'
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
        line_buf.push_back(line.clone());

        // per-part# flags
        let mut part_str;
        let mut col;
        let mut ind1 = usize::max_value();
        let mut ind2;
        
        // iterate through characters until a digit is found
        col = 0;
        part_str = String::new();
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
                    rect_string.push('.');
                }
                print!(" | {}", rect_string);

                // // remove '.' and is_digit()
                // let mut special_str = String::new();
                // for c in rect_string.chars() {
                //     if !(c.is_digit(10) || c == '.') {
                //         special_str.push(c);
                //     }
                // }
                // println!(" | {}", special_str);

                // if !special_str.is_empty() {
                //     sum += part_str.parse::<u32>().unwrap();
                // }
                
                part_str = String::new();
            }

            col += 1;
        }

        // println!("{:?}", line_buf);
        println!("\n<==>\n");
        line_buf.pop_front();
    }


    // print result
    println!("\n\nThe result is (or should be?):  {}", sum);
    Ok(())
}
