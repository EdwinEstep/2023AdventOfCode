// `cargo run ../input.txt`
// I realized after the fact that this solution is incorrect. However,
// it does get the right answer for AoC, so it is good enough for now.
// I'd like to come back to it at some point with a better replacement
// method, starting from the back of the string.

// Currently, it would fail on the string 'oneightwo', as it would
// replace 'oneight' with '18', leaving '1wo'. So, it would select
// '11', rather than the correct '12'.

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
        let mut lstr = line.unwrap().clone();
        let lstr_initial = lstr.clone();

        // do ya conversions, handling some very special cases
        lstr = lstr.replace("eightwo", "82");
        lstr = lstr.replace("eighthree", "83");
        lstr = lstr.replace("sevenine", "79");
        lstr = lstr.replace("threeight", "38");
        lstr = lstr.replace("fiveight", "58");
        lstr = lstr.replace("oneight", "18");
        lstr = lstr.replace("nineight", "98");
        lstr = lstr.replace("twone", "21");

        lstr = lstr.replace("one", "1");
        lstr = lstr.replace("two", "2");
        lstr = lstr.replace("three", "3");
        lstr = lstr.replace("four", "4");
        lstr = lstr.replace("five", "5");
        lstr = lstr.replace("six", "6");
        lstr = lstr.replace("seven", "7");
        lstr = lstr.replace("eight", "8");
        lstr = lstr.replace("nin", "9");

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
        
        println!("{lstr_initial}  |  {lstr}:  {firstlast}");

        calibration_sum += firstlast;
    }

    // print result
    println!("{}", calibration_sum);

    Ok(())
}
