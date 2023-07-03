use std::{io, num::ParseFloatError};

fn main() {
    println!("Welcome to temperature converter!");
    println!("Valid operations:");
    println!("1. Convert fahrenheit to celcius (c)");
    println!("2. Convert celcius to fahrenheit (f)");
    println!("3. Quit (q)");

    loop {
        println!("Input the operation c/f/q");

        let mut operation = String::new();
        io::stdin()
            .read_line(&mut operation)
            .expect("Failed reading line.");

        match operation.trim() {
            "c" => convert_to_celcius(),
            "f" => convert_to_fahrenheit(),
            "q" => break,
            _ => {
                println!("Invalid operation");
                continue;
            }
        }
    }
}

fn convert_to_celcius() {
    loop {
        println!("Input fahrenheit to be converted to celcius");

        let fahrenheit = match get_float_from_user() {
            Ok(num) => num,
            Err(_) => {
                println!("Not a number");
                continue;
            }
        };
        let celcius = (fahrenheit - 32.0) * 0.556;
        println!("{:.1}°C", celcius);
        break;
    }
}

fn convert_to_fahrenheit() {
    loop {
        println!("Input celcius to be converted to fahrenheit");

        let celcius = match get_float_from_user() {
            Ok(num) => num,
            Err(_) => {
                println!("Not a number");
                continue;
            }
        };
        let fahrenheit = (celcius * 1.8) + 32.0;
        println!("{:.1}°F", fahrenheit);
        break;
    }
}

fn get_float_from_user() -> Result<f32, ParseFloatError> {
    let mut float = String::new();
    io::stdin()
        .read_line(&mut float)
        .expect("Failed reading line.");

    return float.trim().parse();
}
