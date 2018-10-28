
extern crate colored;

use std::io::{self, Write};
use colored::*;

mod database;
mod part;
mod terminal;

use database::Database;
use part::Part;
use terminal::ClearCode;

// Operation Codes
enum Opcode {
    Insert, // insert a part into the database
    Search, // search for one part in the database
    Update, // update one part in the database
    Print,  // print all of the parts in the database
    Quit    // quit the program
}

// the status of the previous operation
enum Status {
    Success(String),
    Error(String),
    Neither,
}

use Status::*;

fn main() {
    let mut db = Database::new();
    let mut status: Status = Neither;

    loop {
        terminal::clear(ClearCode::Everything);
        terminal::cursor_move_to_pos(0, 0);
        
        let mut user_input = String::new();

        print_interface();

        match status {
            Success(string) => println!("{}", format!("{}\n", string.trim()).green()),
            Error(string) => println!("{}", format!("{}\n", string.trim()).red()),
            Neither => (),
        }
        status = Neither;

        print!("{}", "Enter an operation code: ".blue());
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut user_input)
            .expect("Failed to read line!");

        let op_code: Opcode = match user_input.trim().parse() {
            Ok(num) => match num {
                1 => Opcode::Insert,
                2 => Opcode::Search,
                3 => Opcode::Update,
                4 => Opcode::Print,
                5 => Opcode::Quit,
                _ => {
                    status = Error("Please enter a valid operation code.".to_string());
                    continue;
                }
            },
            Err(_)  => {
                status = Error("Please enter a valid operation code!".to_string()); 
                continue;
            }
        };

        println!("");

        match op_code {
            Opcode::Insert => {
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
                        status = Error("Please enter a valid part number!".to_string()); 
                        continue;
                    },
                };

                if db.search(part_number).is_some() {
                    status = Error("A part with that ID already exists!".to_string());
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
                        status = Error("[Error]: Please enter a valid part quantity.".to_string());
                        continue;
                    },
                };

                let part = Part {
                    number: part_number,
                    name: part_name,
                    quantity: part_quantity,
                };
                
                db.insert(part);

                status = Success("Part was succesfully inserted!".to_string());
            },

            Opcode::Search => {
                let mut part_number = String::new();

                print!("{}", "Enter part number: ".blue());
                io::stdout().flush().unwrap();
                io::stdin().read_line(&mut part_number)
                    .expect("Failed to read part number!");

                let part_number: u32 = match part_number.trim().parse() {
                    Ok(num) => num,
                    Err(_) => {
                        status = Error("Please enter a valid part number".to_string());
                        continue;
                    }
                };

                let part = match db.search(part_number) {
                    Some(part) => part,
                    None => {
                        status = Error("Could not find the part!".to_string());
                        continue;
                    },
                };

                terminal::clear(ClearCode::Everything);
                terminal::cursor_move_to_pos(0, 0);

                println!("");
                Part::print_part_header();
                part.print();
                println!("");

                let mut discard = String::new();

                println!("{}", "Press ENTER to go back to prompt".red());
                io::stdout().flush().unwrap();
                io::stdin().read_line(&mut discard)
                    .expect("Failed to read user input!");
            },

            Opcode::Update => {
                let mut part_number = String::new();

                print!("{}", "Enter part number: ".blue());
                io::stdout().flush().unwrap();
                io::stdin().read_line(&mut part_number)
                    .expect("Failed to read part number!");

                let part_number: u32 = match part_number.trim().parse() {
                    Ok(num) => num,
                    Err(_) => {
                        status = Error("Please enter a valid part number".to_string());
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
                        status = Error("Please enter a valid part quantity.".to_string());
                        continue;
                    }
                };


                match db.update(part_number, quantity_change) {
                    Ok(updated) => {
                        status = Success(updated);    
                    },
                    Err(error)  => {
                        status = Error(error);
                    },
                };
            }

            Opcode::Print => {
                terminal::clear(ClearCode::Everything);
                terminal::cursor_move_to_pos(0, 0);

                println!("");
                Part::print_part_header();
                for part in db.get_database() {
                    part.print();
                }
                println!("");

                let mut discard = String::new();

                println!("{}", "Press ENTER to go back to prompt".red());
                io::stdout().flush().unwrap();
                io::stdin().read_line(&mut discard)
                    .expect("Failed to read user input!");
            },

            Opcode::Quit => {
                break;
            },
        }
    }
}

fn print_interface () {
    println!("\n{}", "Part Database Management System [PB]".yellow());

    print!("
  1) Insert a part
  2) Search for a part
  3) Update a part
  4) Print the database
  5) Exit the database\n\n"
    );
}

