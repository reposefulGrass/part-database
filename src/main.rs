
extern crate colored;

use std::io::{self, Write};
use colored::*;

mod database;
mod part;

use database::Database;
use part::Part;

// Operation Codes
const INSERT: u32 = 1;
const SEARCH: u32 = 2;
const UPDATE: u32 = 3;
const PRINT:  u32 = 4;
const QUIT:   u32 = 5;

fn main() {
    let mut db = Database::new();

    print_welcome_msg();

    loop {
        let mut user_input = String::new();

        print_interface();

        print!("{}", "Enter an operation code: ".blue());
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut user_input)
            .expect("Failed to read line!");

        let op_code: u32 = match user_input.trim().parse() {
            Ok(num) => num,
            Err(_)  => {
                println!("\n{}", "[PB]: Please enter a valid operation code!".red()); 
                continue;
            }
        };

        println!("");

        match op_code {
            INSERT => {
                let mut part_number   = String::new();
                let mut part_name     = String::new();
                let mut part_quantity = String::new();
            
                print!("{}", "Enter part number: ".blue());
                io::stdout().flush().unwrap();
                io::stdin().read_line(&mut part_number)
                    .expect("Failed to read part number!");
            
                let part_number: u32 = match part_number.trim().parse() {
                    Ok(num) => num,
                    Err(_) => {
                        println!("\n{}", "[PB]: Please enter a valid part number!".red()); 
                        continue;
                    },
                };

                if db.search(part_number).is_some() {
                    println!("\n{}", "[PB]: A part with that ID already exists!".red()); 
                    continue;
                };

                print!("{}", "Enter part name: ".blue());
                io::stdout().flush().unwrap();
                io::stdin().read_line(&mut part_name)
                    .expect("Failed to read part name!");

                // get rid of the trailing newline
                if part_name.contains('\n') {
                    part_name.pop();
                }

                if part_name.len() >= 16 {
                    part_name.truncate(13);
                    part_name.push_str("...");
                }

                print!("{}", "Enter part quantity: ".blue());
                io::stdout().flush().unwrap();
                io::stdin().read_line(&mut part_quantity)
                    .expect("Failed to read part quantity!");

                let part_quantity: u32 = match part_quantity.trim().parse() {
                    Ok(num) => num,
                    Err(_) => {
                        println!("\n{}", "[PB]: Please enter a valid part quantity.".red());
                        continue;
                    },
                };

                let part = Part {
                    number: part_number,
                    name: part_name,
                    quantity: part_quantity,
                };
                
                db.insert(part);
            },

            SEARCH => {
                let mut part_number = String::new();

                print!("{}", "Enter part number: ".blue());
                io::stdout().flush().unwrap();
                io::stdin().read_line(&mut part_number)
                    .expect("Failed to read part number!");

                let part_number: u32 = match part_number.trim().parse() {
                    Ok(num) => num,
                    Err(_) => {
                        println!("\n{}", "[PB]: Please enter a valid part number".red());
                        continue;
                    }
                };

                let part = match db.search(part_number) {
                    Some(part) => part,
                    None => {
                        println!("\n{}", "[PB]: Could not find the part!".red());
                        continue;
                    },
                };

                println!("");
                Part::print_part_header();
                part.print();
            },

            UPDATE => {
                let mut part_number = String::new();

                print!("{}", "Enter part number: ".blue());
                io::stdout().flush().unwrap();
                io::stdin().read_line(&mut part_number)
                    .expect("Failed to read part number!");

                let part_number: u32 = match part_number.trim().parse() {
                    Ok(num) => num,
                    Err(_) => {
                        println!("\n{}", "[PB]: Please enter a valid part number".red());
                        continue;
                    }
                };
                
                let mut quantity_change = String::new();

                print!("{}", "Enter change in quantity: ".blue());
                io::stdout().flush().unwrap();
                io::stdin().read_line(&mut quantity_change)
                    .expect("Failed to read part number!");

                let quantity_change: i32 = match quantity_change.trim().parse() {
                    Ok(num) => num,
                    Err(_) => {
                        println!("\n{}", "[PB]: Please enter a valid part quantity.".red());
                        continue;
                    }
                };


                match db.update(part_number, quantity_change) {
                    Ok(updated)     => println!("{}", updated),
                    Err(error)      => println!("{}", error),
                };
            }

            PRINT => {
                Part::print_part_header();
                for part in db.get_database() {
                    part.print();
                }
            },

            QUIT => {
                break;
            },

            _ => {
                println!("\n{}", "[PB]: Please enter a valid operation code.".red());
                continue;
            }
        }
    }
}

fn print_welcome_msg () {
    println!("\n{}", "Part Database Management System [PB]".yellow());
}

fn print_interface () {
    println!("
  1) Insert a part
  2) Search for a part
  3) Update a part
  4) Print the database
  5) Exit the database\n"
    );
}

