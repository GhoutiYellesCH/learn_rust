use std::env;
use std::process;

fn convert(value: f64, from: char) -> Option<f64> {
    match from {
        'C' => Some(value),                         // Celsius -> Celsius
        'F' => Some((value - 32.0) * 5.0 / 9.0),    // Fahrenheit -> Celsius
        'K' => Some(value - 273.15),                // Kelvin -> Celsius
        _ => None,
    }
}

fn main() {
    // Skip the program name, then collect into Vec<String> and Vec<&str>
    let args: Vec<String> = env::args().skip(1).collect();
    let args: Vec<&str> = args.iter().map(|s| s.as_str()).collect();

    if args.len() != 2 {
        eprintln!("usage: converter <value> <C|F|K>");
        process::exit(1);
    }

    let value: f64 = match args[0].parse() {
        Ok(v) => v,
        Err(_) => {
            eprintln!("error: '{}' is not a number", args[0]);
            process::exit(1);
        }
    };

    let unit = args[1].to_uppercase();

    let from = match unit.as_str() {
        "C" => 'C',
        "F" => 'F',
        "K" => 'K',
        _ => {
            eprintln!("error: unknown unit '{}', use C, F, or K", unit);
            process::exit(1);
        }
    };

    let result = match convert(value, from) {
        Some(celsius) => celsius,
        None => {
            eprintln!("error: unknown unit '{}', use C, F, or K", unit);
            process::exit(1);
        }
    };

    println!("{value:.2}°{unit} = {result:.2}°C");
}