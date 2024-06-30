use std::io;

fn index(a: &[i32], n: i32) -> usize {
    for i in 0..a.len() {
        if a[i] == n {
            return i + 1; 
        }
    }
    0 
}

fn main() {
    let mut k= String::new();
    let mut a = Vec::new();

   
    println!("Enter the length of the array:");
    io::stdin().read_line(&mut k).expect("cannot read line");
    let l: usize = k.trim().parse().expect(" enter a  number");

    
    k.clear();
    println!("Enter the number to find:");
    io::stdin().read_line(&mut k).expect("cannot read line");
    let n: i32 = k.trim().parse().expect(" enter a number");


    println!("Enter array elements:");
    for _ in 0..l {
      k.clear();
        io::stdin().read_line(&mut k).expect("cannot read line");
        let num: i32 = k.trim().parse().expect("enter a number");
        a.push(num);
    }

   
    a.sort();

 
    println!("Sorted array: {:?}", a);

    println!("The index is: {}", index(&a, n));
}
