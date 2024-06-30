use std::io;

fn subarray_sum(a: &[i32]) -> i32 {
    let mut max1 = a[0];
    let mut max2 = a[0];

    for &num in a.iter().skip(1) {
        max2 = i32::max(num, max2 + num);
        max1 = i32::max(max1, max2);
    }

    max1
}

fn main() {
    let mut i = String::new();

    
    println!("Enter array ");
    io::stdin().read_line(&mut i).expect("cannot find");
    let a: Vec<i32> = i
        .trim()
        .split_whitespace()
        .map(|num| num.parse().expect("Invalid"))
        .collect();

    let max = subarray_sum(&a);
    println!("Maximum subarray sum: {}", max);
}
