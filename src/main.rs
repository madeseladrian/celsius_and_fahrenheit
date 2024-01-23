fn main() {
    let fahrenheit_to_celsius: f64 = fahrenheit_to_celsius(32.0);

    println!("The temperature in Celsius is: {fahrenheit_to_celsius}°C");
}

fn fahrenheit_to_celsius(fahrenheit: f64) -> f64 {
    (fahrenheit - 32.0) * 5.0 / 9.0
}