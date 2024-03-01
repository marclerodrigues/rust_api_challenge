mod dto;

use dto::Item;

fn main() {
    let items: Vec<Item> = Vec::new();
    let output = format!("{}", items.len());
    
    println!("{}", output);
}
