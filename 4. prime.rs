use std::io;

fn prime(n: u32) -> bool {
    if n <= 1 {
        return false;
    }
    if n == 2 {
        return true;
    }
    for i in 2..=((n as f64).sqrt() as u32) {
        if n % i == 0 {
            return false;
        }
    }
    true
}

fn main() {
    let mut k = String::new();

    println!("Enter a number:");
    io::stdin().read_line(&mut k).expect("cannot line");
    let n: u32 = k.trim().parse().expect(" enter a number");

    if prime(n) {
        println!("prime");
    } else {
        println!("not prime");
    }
}
