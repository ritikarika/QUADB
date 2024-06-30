use std::io;

fn palindrome(s: &str) -> i32 {
    if s.chars().rev().collect::<String>() == s {
        1
    } else {
        0
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Cannot read line");
    
   
    let input = input.trim();
    
    println!("{}", palindrome(input));
}
