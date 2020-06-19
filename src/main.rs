use std::io;
use std::convert::TryFrom;

const CONVERTERS:[fn(f32) -> f32; 2] = [convert_to_fahrenheit, convert_to_celsius];

fn main() {
    println!(" ");
    println!(":::::::::::::: TEMPERATURE CONVERTER ::::::::::::::");
    println!(" ");

    // conversion options
    // 1: C -> F
    // 2: F -> C
    let option:u32 = get_conversion_type();

    // start conversion for the selected option
    let result:f32 = get_converted_temperature(option);

    // get the output unit
    let unit:char = if option == 0 {
        'F'
    } else {
        'C'
    };

    println!(" ");
    println!("Converted: {}{}", result, unit);
    println!(" ");
}

fn get_conversion_type() -> u32 {
    const C_TO_F: &str = "Celsius to Fahrenheit";
    const F_TO_C: &str = "Fahrenheit to Celsius";
    let options = [C_TO_F, F_TO_C];
    let selected:u32;

    loop {
        println!(" ");
        println!("Enter the conversion type 1 or 2");
        println!(" ");
        println!("1. Celsius to Fahrenheit (default)");
        println!("2. Fahrenheit to Celsius");
        println!(" ");

        let mut conversion_type = String::new();

        io::stdin().read_line(&mut conversion_type)
            .expect("Failed to read!");

        let conversion_type:u32 = match conversion_type.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid option: {}", conversion_type);
                continue;
            }
        };

        if conversion_type != 1 && conversion_type != 2 {
            println!("Invalid option: {}", conversion_type);
            continue;
        }

        selected = conversion_type - 1; // get the array index

        // index has to be converted to usize to access the array elements
        let option = usize::try_from(selected).unwrap();

        // we only need the option variable in order to inform the user what is selected
        println!("You selected: {}", options[option]);
        break;
    }

    // finally we return a valid user selection :)
    selected
}

fn get_converted_temperature(option:u32) -> f32 {
    let result:f32;

    loop {
        println!(" ");
        println!("Enter the temperature:");
        println!(" ");

        let mut temperature = String::new();

        io::stdin().read_line(&mut temperature)
            .expect("Failed to read!");

        let temperature:f32 = match temperature.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid entry! Please enter a number.");
                continue;
            }
        };

        if option == 0 && temperature < -273.15 {
            println!("Lowest valid temperature value in Celsius is {}C", -273.15);
            println!(" ");
            continue;
        }

        if option == 1 && temperature < -459.67 {
            println!("Lowest valid temperature value in Fahrenheit is {}F", -459.67);
            println!(" ");
            continue;
        }

        // index has to be converted to usize to access the array elements
        let option = usize::try_from(option).unwrap();

        result = CONVERTERS[option](temperature);

        break;
    }

    result
}

fn convert_to_fahrenheit(t: f32) -> f32 {
    (t * 9.0 / 5.0) + 32.0
}

fn convert_to_celsius(t: f32) -> f32 {
    (t - 32.0) * 5.0 / 9.0
}
