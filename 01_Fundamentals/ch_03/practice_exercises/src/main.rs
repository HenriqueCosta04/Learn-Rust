use std::io;

fn convert_to_fahrenheit(celsius: f64) -> f64 {
    (celsius * 9.0 / 5.0) + 32.0
}

fn main() {
    let mut input = String::new();
    println!("Enter the temperature in Celsius:");
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let temperature: f64 = input.trim().parse().expect("Please enter a valid number");

    let fahrenheit = convert_to_fahrenheit(temperature);
    println!("The temperature in Fahrenheit is: {fahrenheit}");
}
