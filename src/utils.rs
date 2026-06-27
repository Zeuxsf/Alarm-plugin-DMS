use std::io;
use chrono::NaiveTime;

pub fn input_time() -> Option<NaiveTime> {
    
    println!("Digite o horario: ");

    let mut input: String = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Error");

    match NaiveTime::parse_from_str(input.trim(), "%H:%M") {
        Ok(time) => Some(time),
        Err(_) => {
            println!("Invalid Time.");
            None
        }
        
    }
}

pub fn input() -> String {
    let mut input: String = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Error");

    input

    // let num: i32 = match input.trim().parse() {
    //     Ok(n) => n,
    //     Err(_) => {
    //         println!("Invalid Number");
    //         return 0;
    //     }
    // };
}

pub fn input_menu() -> u8 {
    let mut input: String = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Error");

    let num:u8  = match input.trim().parse() {
        Ok(n) => n,
        Err(_) => {
            println!("Invalid Number");
            return 0;
        }
    };

    return num;
}