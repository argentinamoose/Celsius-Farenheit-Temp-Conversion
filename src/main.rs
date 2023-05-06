use std::io;
use std::process::exit;

// Multiplication constants for conversion
const FC:f32 = 5.0/9.0;
const CF:f32 = 9.0/5.0;

fn leave() -> i32 {
    1
}

fn main() {
    let exit_code = leave();
    
    // A question for if you are using f or c
    println!("Would you like to start from Farenheit or Celsius?");
    let mut unit = String::new();

    io::stdin()
        .read_line(&mut unit)
        .expect("Failed to read line");

    let input_type:String = unit.trim().parse().expect("Input not an number");

    // Asks for your temperature
    println!("Enter your Temperature");
    let mut input_line = String::new();

    io::stdin()
        .read_line(&mut input_line)
        .expect("Failed to read line");

    // Takes number out of input
    let start_temp:f32 = input_line.trim().parse().expect("Input not an number");
    
    // List of values to check if user asked for farenheit
    let a = [
        String::from("Farenheit"),
        String::from("farenheit"),
        String::from("F"),
        String::from("f"),
    ];

    // Checks for unit, does math, and writes the converted temp
    for element in a {
        if input_type == element{
            let pre_final:f32 = (start_temp - 32.0) * FC;
            let end_temp = pre_final as i32;
            println!("Celsius: {end_temp}");
            exit(exit_code);
        }
    }
    let pre_final:f32 = start_temp * CF + 32.0;
    let end_temp = pre_final as i32;
    println!("Farenheit: {end_temp}");
}