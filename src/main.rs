extern crate dialoguer;

use dialoguer::{theme::ColorfulTheme, Select};

use std::io::{self, Read};

fn main() {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer).unwrap();
    let selections = buffer.split("\n").collect::<Vec<_>>();

    let selection = Select::with_theme(&ColorfulTheme::default())
        .default(0)
        .items(&selections[..])
        .interact()
        .unwrap();

    println!("{}", selections[selection]);
}
