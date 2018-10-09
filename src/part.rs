
extern crate colored; 

use colored::*;

pub struct Part {
    pub number: u32,
    pub name: String,
    pub quantity: u32,
}

impl Part {
    pub fn print_part_header () {
        println!("{}", "Part Number         Part Name           Quantity on Hand".green());
        println!("{}", "--------------------------------------------------------".green());
    }
    
    pub fn print (&self) {
        let part_components = format!(
            "{:<16}    {:<16}    {:<16}",
            self.number,
            &self.name,
            self.quantity,
        );

        println!("{}", part_components.green());
    }
}
