use std::io;

fn sorted_arrays(a1: &[i32], a2: &[i32]) -> Vec<i32> {
    let mut m= Vec::with_capacity(a1.len() + a2.len());
    let (mut i, mut j) = (0, 0); 

    while i < a1.len() && j < a2.len() {
        if a1[i] <= a2[j] {
            m.push(a1[i]);
            i += 1;
        } else {
            m.push(a2[j]);
            j += 1;
        }
    }

    while i < a1.len() {
        m.push(a1[i]);
        i += 1;
    }

    while j < a2.len() {
        m.push(a2[j]);
        j += 1;
    }

    m
}

fn main() {
    let mut i = String::new();

    println!("Enter sorted array:");
    io::stdin().read_line(&mut i).expect("cannot read");
    let a1: Vec<i32> = i
        .trim()
        .split_whitespace()
        .map(|num| num.parse().expect("Invalid"))
        .collect();
    
    i.clear(); 

    println!("Enter sorted array :");
    io::stdin().read_line(&mut i).expect("cannot read");
    let a2: Vec<i32> = i
        .trim()
        .split_whitespace()
        .map(|num| num.parse().expect("Invalid"))
        .collect();
    let m = sorted_arrays(&a1, &a2);

    println!("Merged array: {:?}", m);
}
