use std::io;

fn small_word(s: &str) -> Option<&str> {
    let mut words = s.split_whitespace();

    
    let first = match words.next() {
        Some(w) => w,
        None => return None,
    };
    
    // Initialize variables to track the shortest word and its length
    let mut small = first;
    let mut small_len = small.len();
    
    // Iterate through the words to find the shortest one
    for w in words {
        let w_len = w.len();
        if w_len < small_len {
            small = w;
            small_len = w_len;
        }
    }
    
    Some(small)
}

fn main() {
    let mut i = String::new();
    
    println!("Enter the string:");
    io::stdin().read_line(&mut i).expect("Cannot read line");
    let s = i.trim();
    
    if let Some(small) = small_word(s) {
        println!("Shortest word: {}", small);
    } else {
        println!("No words found.");
    }
}
