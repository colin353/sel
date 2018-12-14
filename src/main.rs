extern crate dialoguer;

use dialoguer::{theme::ColorfulTheme, Select};

use std::io::{self, Read};

fn main() {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer).unwrap();
    let selections = buffer.split("\n").map(|s| s.trim()).filter(|s| !s.is_empty()).collect::<Vec<_>>();

    let selection = match selections.len() {
        // No items to select from.
        0 => return,
        // Only one item - just immediately return that.
        1 => 0,
        // Choose from a menu.
        _ => Select::with_theme(&ColorfulTheme::default())
                .default(0)
                .items(&selections[..])
                .interact()
                .unwrap()
    }

    println!("{}", selections[selection]);
}
