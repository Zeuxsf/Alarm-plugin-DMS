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