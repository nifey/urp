mod cube;
mod cubelist;

use cubelist::CubeList;
use std::collections::HashMap;
use std::env;
use std::io::{self, BufRead, Read};

fn main() {
    let mut index = HashMap::<u32, CubeList>::new();
    if env::args().collect::<Vec<String>>().len() > 1 {
        // Read commands from file
        let mut file =
            std::fs::File::open(env::args().collect::<Vec<String>>()[1].as_str()).unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents);
        for line in contents.lines() {
            if !handle_command(&mut index, line.to_string()) {
                return;
            }
        }
    } else {
        // Read commands from standard input
        let stdin = io::stdin();
        for line in stdin.lock().lines() {
            if !handle_command(&mut index, line.unwrap()) {
                return;
            }
        }
    }
}

/// This function handles a single command
/// It returns false if the command is a quit command, else it returns true
fn handle_command(index: &mut HashMap<u32, CubeList>, line: String) -> bool {
    let contents: Vec<&str> = line.split_whitespace().collect();
    let command: char = contents[0].chars().collect::<Vec<char>>()[0];
    match command {
        'q' => return false,
        'p' => {
            // Prints the function into a file
            if contents.len() < 2 {
                println!("Expected 1 argument");
                return true;
            }
            let function: u32 = contents[1].parse::<u32>().expect("Expected an number");
            if let Some(cubelist_1) = index.get(&function) {
                cubelist_1.write_to_file(format!("{}.pcn", function).as_str());
            }
        }
        't' => {
            // Checks if a function is a tautology
            if contents.len() < 2 {
                println!("Expected 1 argument");
                return true;
            }
            let function: u32 = contents[1].parse::<u32>().expect("Expected an number");
            if let Some(cubelist_1) = index.get(&function) {
                if cubelist_1.is_tautology() {
                    println!("{} is a tautology", function);
                } else {
                    println!("{} is not a tautology", function);
                }
            }
        }
        'r' => {
            // Reads the function from a file
            if contents.len() < 2 {
                println!("Expected 1 argument");
                return true;
            }
            let function: u32 = contents[1].parse::<u32>().expect("Expected an number");
            index.insert(
                function,
                CubeList::read_from_file(format!("{}.pcn", function).as_str()),
            );
        }
        '!' => {
            // Perform Complement of a function
            if contents.len() < 3 {
                println!("Expected 2 arguments");
                return true;
            }
            let output: u32 = contents[1].parse::<u32>().expect("Expected an number");
            let input: u32 = contents[2].parse::<u32>().expect("Expected an number");
            if let Some(cubelist_1) = index.get(&input) {
                index.insert(output, cubelist_1.complement());
            } else {
                println!("Boolean function {} not found", input);
            }
        }
        '&' => {
            // Perform AND of two functions
            if contents.len() < 4 {
                println!("Expected 3 arguments");
                return true;
            }
            let output: u32 = contents[1].parse::<u32>().expect("Expected an number");
            let input1: u32 = contents[2].parse::<u32>().expect("Expected an number");
            let input2: u32 = contents[3].parse::<u32>().expect("Expected an number");
            if let Some(cubelist_1) = index.get(&input1) {
                if let Some(cubelist_2) = index.get(&input2) {
                    index.insert(output, cubelist_1.and(cubelist_2));
                } else {
                    println!("Boolean function {} not found", input2);
                }
            } else {
                println!("Boolean function {} not found", input1);
            }
        }
        '+' => {
            // Perform OR of two functions
            if contents.len() < 4 {
                println!("Expected 3 arguments");
                return true;
            }
            let output: u32 = contents[1].parse::<u32>().expect("Expected an number");
            let input1: u32 = contents[2].parse::<u32>().expect("Expected an number");
            let input2: u32 = contents[3].parse::<u32>().expect("Expected an number");
            if let Some(cubelist_1) = index.get(&input1) {
                if let Some(cubelist_2) = index.get(&input2) {
                    index.insert(output, cubelist_1.or(cubelist_2));
                } else {
                    println!("Boolean function {} not found", input2);
                }
            } else {
                println!("Boolean function {} not found", input1);
            }
        }
        _ => {
            println!("Invalid command");
        }
    }
    true
}
