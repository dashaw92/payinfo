use std::fs::read_to_string;

use crate::table::EventTable;

mod table;

fn main() {
    let Some(file) = std::env::args().skip(1).nth(0) else {
        eprintln!("Please pass a file");
        return
    };
    
    let Ok(stub) = read_to_string(file) else {
        eprintln!("Failed to read file");
        return
    };

    let stub = EventTable::parse_stub(&stub);
    println!("{}", stub.to_csv());
}
