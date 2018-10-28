
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

#[test]
fn db_insert_test () {
    let mut db = Database::new();
    let part = Part { number: 123, name: "Metal".to_string(), quantity: 100 };

    db.insert(part);

    assert!(db.database.len() == 1);
}

#[test]
fn db_search_test () {
    let mut db = Database::new();
    let part = Part { number: 123, name: "Metal".to_string(), quantity: 100 };

    db.insert(part);

    let part2 = db.search(123).unwrap();

    assert_eq!(123, part2.number);
    assert_eq!("Metal".to_string(), part2.name);
    assert_eq!(100, part2.quantity);
}

#[test]
fn db_update_test () {
    let mut db = Database::new();
    let part = Part { number: 123, name: "Metal".to_string(), quantity: 100 };

    db.insert(part);
    db.update(123, -50);

    let part2 = db.search(123).unwrap();

    assert_eq!(123, part2.number);
    assert_eq!("Metal".to_string(), part2.name);
    assert_eq!(50, part2.quantity);
}




