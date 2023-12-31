// `cargo run ../input.txt`

// for cmd-line args
use std::env;

// files
use std::fs::File;
use std::io::{prelude::*, BufReader}; // for reading line-by-line

fn main() -> std::io::Result<()> {
    // get file name from input args
    let args: Vec<String> = env::args().collect();
    let fpath = &args[1];

    // open file and set up buff reader
    // 8KB buffer size by default.
    // Or init with `with_capacity(num_bytes, file)` instead of `new()`
    let file = File::open(fpath)?;
    let reader = BufReader::new(file);  

    // useful vars
    let mut calibration_sum: u32 = 0;

    // find the largest value, igorning empty strings
    for line in reader.lines() {
        let lstr = line.unwrap().clone();

        let mut firstlast: String = String::new();
        let mut first: char = '_';
        let mut last: char = '_';

        for c in lstr.chars() {
        // pull out the digits from the line
            if c.is_digit(10) {
                if !(first.is_digit(10)) {
                    first = c;
                }
                
                // set last to each char until line is over
                last = c;
            }
        }


        // build integer and add to sum
        firstlast.push(first);
        firstlast.push(last);
        let firstlast: u32 = firstlast.parse().expect("hi there :)");
        
        calibration_sum += firstlast;
    }

    // print result
    println!("{}", calibration_sum);

    Ok(())
}
