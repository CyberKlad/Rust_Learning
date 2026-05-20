use std::io;

fn main() {
    let mode: bool;

    loop {
        println!("choose converter:");
        println!("(1) Fahrenheit to Celsius");
        println!("(2) Celsius to Fahrenheit");

        let mut answer = String::new();

        io::stdin()
            .read_line(&mut answer)
            .expect("[Error] failed to read line");

        let answer: u8 = match answer.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!(
                    "[Error] could not convert {} to a u8 value",
                    answer.trim_end()
                );
                continue;
            }
        };


        if answer == 1 {
            mode = true;
            break;
        } else if answer == 2 {
            mode = false;
            break;
        } else {
            println!("[Error] invalid answer: {}", answer);
            continue;
        }
    }
    loop {
        if mode {
            println!("Enter value to be converted from Fahrenheit to Celsius");
        } else {
            println!("Enter value to be converted from Celsius to Fahrenheit");
        }

        let mut answer = String::new();

        io::stdin()
            .read_line(&mut answer)
            .expect("[Error] failed to read line");

        let answer: f32 = match answer.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("[Error] could not convert {} to a f32 value", answer);
                continue;
            }
        };

        if mode {
            let celsius = (answer - 32.0) * (5.0 / 9.0);
            println!("{} Fahrenheit is equal to {} Celsius", answer, celsius);
            break;
        } else {
            let fahrenheit = answer * (9.0 / 5.0) + 32.0;
            println!("{} Celsius is equal to {} Fahrenheit", answer, fahrenheit);
            break;
        }
    }
}
