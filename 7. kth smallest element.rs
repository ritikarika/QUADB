// Function to find the kth smallest element in the array
use std::io;


fn smallest(a: &[i32], k: usize) -> Option<i32> {
    if k == 0 || k > a.len() {
        return None;
    }


    let mut sorted= a.to_vec();
    sorted.sort();

    
    Some(sorted[k - 1])
}

fn main() {
    let mut i = String::new();

    println!("Enter array elements :");
    io::stdin().read_line(&mut i).expect("cannot read line");

    let a: Vec<i32> = i
        .trim()
        .split_whitespace()
        .map(|num| num.parse().expect("Invalid"))
        .collect();

    println!("Enter k:");
    i.clear();
    io::stdin().read_line(&mut i).expect("cannot read line");
    let k: usize = i.trim().parse().expect(" enter a  number");

    if let Some(result) =smallest(&a, k) {
        println!("The {}th smallest element in the array is: {}", k, result);
    } else {
        println!("Invalid k value or array is empty.");
    }
}
