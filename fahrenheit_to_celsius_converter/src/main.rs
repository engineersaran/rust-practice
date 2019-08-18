use std::io;
fn main() {
    println!("Fahrenheit to Celsius Converter!");
    println!("--------------------------------");
    println!("Please enter the Fahrenheit value");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    let fahrenheit: f32 = match input.trim().parse() {
        Ok(num) => num,
        Err(_) => 0.0,
    };
    if fahrenheit == 0.0 {
        println!("Invalid input");
    } else {
        println!("The Fahrenheit value entered is: {}", fahrenheit);
        let celsius: f32 = (fahrenheit - 32.0) * (5.0 / 9.0);
        println!("The celsius value is: {}", celsius);
    }
}
