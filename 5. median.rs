use std::io;

fn median(a: &[f64]) -> Option<f64> {
    let n = a.len();

    if n == 0 {
        return None; 
    }

    let mid = n / 2;

    if n % 2 == 0 {
        
        let median = (a[mid - 1] + a[mid]) / 2.0;
        Some(median)
    } else {
        
        Some(a[mid])
    }
}

fn main() {
    let mut input = String::new();

    println!("Enter array elements (space-separated floats):");
    io::stdin().read_line(&mut input).expect("cannot read line");

    let a: Vec<f64> = input.trim()
        .split_whitespace()
        .map(|num| num.parse().expect("Invalid "))
        .collect();

    if let Some(median) = median(&a) {
        println!("Median: {}", median);
    } else {
        println!("No median found because the array is empty.");
    }
}
