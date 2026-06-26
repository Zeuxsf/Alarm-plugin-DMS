use chrono::{DateTime, Local, Duration, NaiveTime};
use notify_rust::Notification; 
use serde::{Serialize, Deserialize}; 
use std::fs;
use std::thread;
use std::time::Duration as StdDuration;
mod utils;

#[derive(Serialize, Deserialize)]
struct Alarme {
    mensagem: String,
    categoria: String, 
    a_delete: bool,
    horario: NaiveTime,
}

// trait Notificacao {
//     fn notificacao(&self);
// }

impl Alarme {
    fn notificacao(&self) {
        println!("nome: {}", self.mensagem);
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {

    let horario = match utils::input_time() {
        Some(time) => time,
        None => return Ok(()),
    };

    let ovo: Alarme = Alarme {
        mensagem: String::from("Tome aguaaaaa"),
        categoria: String::from("Comida"),
        a_delete: true,
        horario
    };

    println!("Alarme: {}. Hora salva: {}", ovo.mensagem, ovo.horario);
    ovo.notificacao();

    Ok(())
}
