fn main() {
    let fahrenheit_to_celsius: f64 = fahrenheit_to_celsius(32.0);
    println!("The temperature in Celsius is: {fahrenheit_to_celsius}°C");
    let celsius_to_fahrenheit: f64 = celsius_to_fahrenheit(0.0);
    println!("The temperature in Fahrenheit is: {celsius_to_fahrenheit}°F");
}

fn celsius_to_fahrenheit(celsius: f64) -> f64 {
    celsius * 9.0 / 5.0 + 32.0
}

fn fahrenheit_to_celsius(fahrenheit: f64) -> f64 {
    (fahrenheit - 32.0) * 5.0 / 9.0
}
