use std::io;

fn main() {
    println!("Convert Celsius to Fahrenheit--");

    let mut guess = String::new();

    println!("Please Enter Celcius");

    io::stdin()
        .read_line(&mut guess)
        .expect("Unable to read line");

    let guess: f64 = match guess.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid Number");
            0.0
        }
    };

    // let fahrenheit = convert_celsius_to_fahrenheit(&guess);
    let celcius = convert_fahrenheit_to_celsius(&guess);
    println!("Temprature in Fahrenheit is: {}", &celcius);
    // println!("Temprature in Fahrenheit is: {}", fahrenheit);
}

fn convert_celsius_to_fahrenheit(celsius: &f32) -> f32 {
    (celsius * 1.8) + 32.0
}

fn convert_fahrenheit_to_celsius(fahrenheit: &f64) -> f64 {
    let factor = 5 as f64 / 9 as f64;
    (fahrenheit - 32 as f64) * factor
}
