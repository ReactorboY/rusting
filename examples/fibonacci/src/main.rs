use std::io;

fn main() {
    println!("Let's Print the Fibonacci Series");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Unable to read line");

    let guess: u32 = match guess.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            print!("Invalid Number");
            0
        }
    };

    if guess != 0 {
        fibonacci_series(&guess);
    }
}

fn fibonacci_series(num: &u32) {
    let mut prev: u32 = 0;
    let mut cur: u32 = 1;

    println!("First Number is: {}", prev);
    println!("Second Number is: {}", cur);

    for _ in 1..*num {
        let sum = prev + cur;
        prev = cur;
        cur = sum;
        println!("Next Number is: {}", sum);
    }
}
