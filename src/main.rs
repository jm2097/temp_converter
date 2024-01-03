use colored::Colorize;
use std::io;

fn main() {
    let temp = prompt_for_temp();
    let mode = prompt_for_mode();

    match mode {
        1 => {
            let c = fahrenheit_to_celsius(temp);
            println!("{}", format!("\n{}°F = {:.3}°C", temp, c).bright_green())
        }
        2 => {
            let f = celsius_to_fahrenheit(temp);
            println!("{}", format!("\n{}°C = {:.3}°F", temp, f).bright_green())
        }
        _ => {
            println!("{}", "Invalid mode".yellow());
            return ();
        }
    };
}

fn prompt_for_temp() -> f32 {
    let mut temp = String::new();

    loop {
        println!("{}", "\nEnter temperature:".bright_blue());
        temp.clear();
        io::stdin()
            .read_line(&mut temp)
            .expect("Failed to read line");

        match temp.trim().parse::<f32>() {
            Ok(t) => return t,
            Err(_) => println!(
                "{}",
                "Invalid input. Please enter a number for temperature.".yellow()
            ),
        }
    }
}

fn prompt_for_mode() -> u8 {
    let mut mode = String::new();

    loop {
        println!("{}", "\nEnter conversion mode:".bright_blue());
        println!("{}", "  1. Convert from °F to °C".bright_purple());
        println!("{}", "  2. Convert from °C to °F".bright_purple());
        mode.clear();
        io::stdin()
            .read_line(&mut mode)
            .expect("Failed to read line");

        let mode = match mode.trim().parse::<u8>() {
            Ok(m) => m,
            Err(_) => {
                println!(
                    "{}",
                    "Invalid mode. Please enter 1 for °F to °C or 2 for °C to °F.".yellow()
                );
                continue;
            }
        };

        if mode != 1 && mode != 2 {
            println!(
                "{}",
                "Invalid mode. Please enter 1 for °F to °C or 2 for °C to °F.".yellow()
            );
            continue;
        }

        return mode;
    }
}

/// Converts a temperature from Fahrenheit to Celsius.
///
/// # Arguments
///
/// * `f` - A f32 that holds the temperature in Fahrenheit.
///
/// # Formula
///
/// (°F - 32) × 5/9 = °C
fn fahrenheit_to_celsius(f: f32) -> f32 {
    let c = (f - 32.0) * 5.0 / 9.0;
    c
}

/// Converts a temperature from Celsius to Fahrenheit.
///
/// # Arguments
///
/// * `c` - A f32 that holds the temperature in Celsius.
///
/// # Formula
///
/// (°C × 9/5) + 32 = °F
fn celsius_to_fahrenheit(c: f32) -> f32 {
    let f = (c * 9.0 / 5.0) + 32.0;
    f
}

#[cfg(test)]
mod tests {
    use crate::{celsius_to_fahrenheit, fahrenheit_to_celsius};

    #[test]
    fn test_fahrenheit_to_celsius() {
        assert_eq!(fahrenheit_to_celsius(32.0), 0.0);
        assert_eq!(fahrenheit_to_celsius(212.0), 100.0);
        assert_eq!(fahrenheit_to_celsius(-40.0), -40.0);
        assert_eq!(fahrenheit_to_celsius(50.0), 10.0);
        assert_eq!(fahrenheit_to_celsius(86.0), 30.0);
    }

    #[test]
    fn test_celsius_to_fahrenheit() {
        assert_eq!(celsius_to_fahrenheit(0.0), 32.0);
        assert_eq!(celsius_to_fahrenheit(100.0), 212.0);
        assert_eq!(celsius_to_fahrenheit(-40.0), -40.0);
        assert_eq!(celsius_to_fahrenheit(10.0), 50.0);
        assert_eq!(celsius_to_fahrenheit(30.0), 86.0);
    }
}
