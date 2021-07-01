use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    // grabs input from input file
    if let Ok(lines) = read_lines("C:/Advent/1/input.txt") {
        // Consumes the iterator, returns a string, assignes outer name for loop
        'outer: for line in lines {
            if let Ok(stars1) = line {
                //repeats the grab from input
                if let Ok(rows) = read_lines("C:/Advent/1/input.txt") {
                    for row in rows {
                        if let Ok(stars2) = row {
                            //sends the strings over to the add function, if they add to 2020, multiply then break out
                            let my_int = add(&stars1, &stars2);
                                if my_int == 2020 {
                                    let my_mult = multiply(&stars1, &stars2);
                                    println!("{}", &my_mult);
                                    break 'outer;
                                }
                        }
                    }
                }
            }
        }
    }
}

fn add(first_number_str: &String, second_number_str: &String) -> i32 {
    //unwrap the number from string to i32
    let first_number = first_number_str.parse::<i32>().unwrap();
    let second_number = second_number_str.parse::<i32>().unwrap();
    first_number + second_number
}
fn multiply(first_number_str: &String, second_number_str: &String) -> i32 {
    //unwrap the number from string to i32
    let first_number = first_number_str.parse::<i32>().unwrap();
    let second_number = second_number_str.parse::<i32>().unwrap();
    first_number * second_number
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
    where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}