use std::io;

fn main() {
    let mut celsius_str = String::new();

    println!("Enter a temperature in Celsius: ");
    io::stdin()
        .read_line(&mut celsius_str)
        .expect("Failed to read line");

    let celsius: f64 = celsius_str.trim().parse().expect("Please type a number!");

    let fahrenheit = celsius * 1.8 + 32.0;

    println!("The temperature in Fahrenheit is: {fahrenheit}");
}
