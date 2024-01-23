use std::io;

fn main() {
    let temperature = get_temperature_input();

    let fahrenheit_to_celsius: f64 = fahrenheit_to_celsius(temperature);
    println!("The temperature in Celsius is: {fahrenheit_to_celsius}°C");
}

fn get_temperature_input() -> f64 {
    loop {
        println!("Enter with temperature: ");
        
        let mut input = String::new();

        io::stdin().read_line(&mut input).expect("Failed to read line");

        match input.trim().parse() {
            Ok(number) => return number,
            Err(_) => {
                println!("Invalid input. Please enter a valid number.");
                continue;
            } 
        }
    }
}

// fn celsius_to_fahrenheit(celsius: f64) -> f64 {
//     celsius * 9.0 / 5.0 + 32.0
// }

fn fahrenheit_to_celsius(fahrenheit: f64) -> f64 {
    (fahrenheit - 32.0) * 5.0 / 9.0
}
