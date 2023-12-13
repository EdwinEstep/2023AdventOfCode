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
    let mut id_sum: u32 = 0;
    let mut impossible_game = false;

    // find the largest value, igorning empty strings
    for line in reader.lines() {
        let lstr = line.unwrap().clone();

        let mut split_str = lstr.split_whitespace();

        // remove first chunk, as it pertains to unused data
        split_str.next();
        let mut thing_string = split_str.next().unwrap().replace(":", "");

        let mut game_id: u32 = thing_string.parse::<u32>().unwrap();
        let mut prev_digit = 0;
        impossible_game = false;

        for thing in split_str {
            // determine whether it's a digit. If not, use the previous digit
            let mut thing_string = thing.replace(",", "");
            let mut thing_string = thing_string.replace(";", "");
            let mut test = thing_string.parse::<u32>();

            match test {
                Ok(ok) => {
                    prev_digit = ok;
                    print!("{} ", ok);
                },
                Err(_) => {
                    println!("{}", thing_string);

                    match thing_string.as_ref() {
                        "red" => {
                            if prev_digit > 12  {
                                impossible_game = true;
                            }
                        },
                        "green" => {
                            if prev_digit > 13  {
                                impossible_game = true;
                            }
                        },
                        "blue" => {
                            if prev_digit > 14  {
                                impossible_game = true;
                            }
                        },
                        _ => {
                            println!("Not what you expected, pal");
                        }
                    }
                }, 
            }  
        }

        if impossible_game {
            println!("Game {}:  Impossible!", game_id);
        }
        else {
            println!("Game {}:  Possible!", game_id);

            id_sum += game_id;
        }        
    }

    // print result
    println!("{}", id_sum);

    Ok(())
}
