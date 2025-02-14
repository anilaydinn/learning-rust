use std::io;

fn main() {
    let mut n = String::new();

    println!("Enter a number: ");
    io::stdin().read_line(&mut n).expect("Failed to read line");

    let n: u32 = n.trim().parse().expect("Please type a number!");
    println!("The {}th Fibonacci number is {}", n, fibonacci(n));
}

fn fibonacci(n: u32) -> u32 {
    if n <= 1 {
        return n;
    }
    fibonacci(n - 1) + fibonacci(n - 2)
}
