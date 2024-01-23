use std::io;

fn main() {
    println!("Temperature Converter");

    loop {
        let choice: u32 = choice();

        if choice == 1 {
            let temperature = get_temperature_input("Enter with the temperature in Fahrenheit: ");
            let fahrenheit_to_celsius: f64 = fahrenheit_to_celsius(temperature);
            println!("The temperature in Celsius is: {fahrenheit_to_celsius}°C");
            println!("")
        } else if choice == 2 {
            let temperature = get_temperature_input("Enter with the temperature in Celsius: ");
            let celsius_to_fahrenheit: f64 = celsius_to_fahrenheit(temperature);
            println!("The temperature in Celsius is: {celsius_to_fahrenheit}°F");
            println!("");
        } else if choice == 3 {
            println!("Exiting the program!");
            println!("");
            break;
        } else {
            println!("Invalid choice. Please enter a valid option (1-3).");
            println!();
        }
    }
}

fn choice() -> u32 {
    loop {
        println!("Choose an option: ");
        println!("1. Fahrenheit to Celsius");
        println!("2. Celsius to Fahrenheit");
        println!("3. Exit");

        let mut choice = String::new();

        io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read line.");

        match choice.trim().parse() {
            Ok(number) => return number,
            Err(_) => {
                println!("Invalid input. Please enter a number!");
                continue;
            }
        }
    }
}

fn get_temperature_input(prompt: &str) -> f64 {
    loop {
        println!("{}", prompt);

        let mut input = String::new();

        io::stdin().read_line(&mut input).expect("Failed to read line.");

        match input.trim().parse() {
            Ok(number) => return number,
            Err(_) => {
                println!("Invalid input. Please enter a valid number.");
                continue;
            } 
        }
    }
}

fn celsius_to_fahrenheit(celsius: f64) -> f64 {
    celsius * 9.0 / 5.0 + 32.0
}

fn fahrenheit_to_celsius(fahrenheit: f64) -> f64 {
    (fahrenheit - 32.0) * 5.0 / 9.0
}
