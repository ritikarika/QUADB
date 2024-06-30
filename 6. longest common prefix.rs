// Function to find the longest common prefix among a set
use std::io;

fn longest_common_prefix(s: &[String]) -> String {
    if s.is_empty() {
        return String::new(); 
    }

    let mut pre = s[0].clone(); 

    / Iterate through each string in the vector (except the first one)
    for i in &s[1..] {
        
        while !i.starts_with(&pre) {
            pre.pop(); 
            if pre.is_empty() {
                return String::new(); 
            }
        }
    }

    pre
}

fn main() {
    let mut k = String::new();

    
    println!("Enter strings (space-separated):");
    io::stdin().read_line(&mut k).expect("cannot read line");

    let str: Vec<String> = k.split_whitespace().map(String::from).collect();

    let lcp = longest_common_prefix(&str);
    println!("Longest common prefix: {}", lcp);
}
