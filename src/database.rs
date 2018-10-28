
use part::Part;

pub struct Database {
    pub database: Vec<Part>,
}

impl Database {
    pub fn new () -> Database {
        Database {
            database: Vec::new()    
        }
    }

    pub fn insert (&mut self, part: Part) {
        self.database.push(part);     
    }

    pub fn search (&mut self, number: u32) -> Option<&mut Part> {
        for part in self.get_mut_database() {
            if part.number == number {
                return Some(part);
            }
        }

        None
    }

    pub fn update (&mut self, number: u32, quantity_change: i32) -> Result<String, String> {
        let found = match self.search(number) {
            Some(part) => part,
            None => {
                return Err("\nPart was not found!".to_string());
            }
        };

        // make sure that quantity += quantity_change isn't negative
        if quantity_change.is_negative() {
            let new_quantity = found.quantity as i32 + quantity_change;

            // prevent the quantity from being negative
            if new_quantity.is_negative() {
                return Err("\nQuantity cannot be negative.".to_string());
            }
            else {
                found.quantity = new_quantity as u32;
            }
        }
        else {
            found.quantity += quantity_change as u32;
        }

        Ok("\nPart was updated!".to_string())
    }

    pub fn get_database (&self) -> &Vec<Part> {
        &self.database
    }

    pub fn get_mut_database (&mut self) -> &mut Vec<Part> {
        &mut self.database
    }
}
