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
    let mut power_sum: u32 = 0;

    // find the largest value, igorning empty strings
    for line in reader.lines() {
        let lstr = line.unwrap().clone();

        let mut split_str = lstr.split_whitespace();

        // remove first chunk, as it pertains to unused data
        split_str.next();
        let thing_string = split_str.next().unwrap().replace(":", "");

        let game_id: u32 = thing_string.parse::<u32>().unwrap();
        let mut min_red = 0;
        let mut min_green = 0;
        let mut min_blue = 0;
        let mut prev_digit = 0;

        for thing in split_str {
            // determine whether it's a digit. If not, use the previous digit
            let thing_string = thing.replace(",", "");
            let thing_string = thing_string.replace(";", "");
            let test = thing_string.parse::<u32>();

            match test {
                Ok(ok) => {
                    prev_digit = ok;
                    print!("{} ", ok);
                },
                Err(_) => {
                    println!("{}", thing_string);

                    match thing_string.as_ref() {
                        "red" => {
                            if prev_digit > min_red  {
                                min_red = prev_digit;
                            }
                        },
                        "green" => {
                            if prev_digit > min_green  {
                                min_green = prev_digit;
                            }
                        },
                        "blue" => {
                            if prev_digit > min_blue  {
                                min_blue = prev_digit;
                            }
                        },
                        _ => {
                            println!("Not what you expected, pal");
                        }
                    }
                }, 
            }  
        }

        println!("Game {game_id}:  r={min_red} g={min_green} b={min_blue}");

        power_sum += min_red*min_green*min_blue;
    }

    // print result
    println!("{}", power_sum);

    Ok(())
}
