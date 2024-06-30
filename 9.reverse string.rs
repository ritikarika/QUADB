//reverse a string

use std::io;

fn reverse(s: &str) -> String {
    s.chars().rev().collect::<String>()
}

fn main() {
    let mut i = String::new();

    println!("Enter a string:");
    io::stdin().read_line(&mut i)
        .expect("cannot read line");

    let trimmed = i.trim(); 
    let reversed = reverse(trimmed);
    println!("Reversed: {}", reversed);
}
