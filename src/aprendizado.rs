use chrono::{DateTime, Local, Duration};
use notify_rust::Notification; // Importa a struct de notificação que manda a notificação pro sistema
use serde::{Serialize, Deserialize}; // Importa duas traits do serde. Serialize: converte uma struct para JSON. Deserialize: converte JSON para struct
use std::fs; // Importa o módulo filesystem da lib padrão do rust. Usado para ler e escrever arquivos por código
use std::thread;
use std::time::Duration as StdDuration;


#[derive(Serialize, Deserialize)] // Fala pro compilador gerar automaticamente o código de serialize e desserialize para essa struct, sem isso não é possível converter   
struct Alarme {
    id: i32,
    msg: String,
    ativo: bool,
    horario: DateTime<Local>
}

pub fn teste() -> Result<(), Box<dyn std::error::Error>> { // Retorna um result para poder usar "?" nas operações que podem falhar. Se der erro, o programa mostra

    let horario = Local::now() + Duration::minutes(1) ;

    let ovo: Alarme = Alarme { id: 1, msg: String::from("Tome aguaaaaa"), ativo: true, horario,}; // Cria uma instancia da struct alarme

    let json = serde_json::to_string_pretty(&ovo).unwrap(); // Converte a struct para JSON 
    
    fs::write("alarmes.json", &json).unwrap();
    println!("{}", json); // Salva o arquivo JSON

    let conteudo = fs::read_to_string("alarmes.json")?; // Lê o arquivo e retorna o conteúdo como String

    let lembrete: Alarme = serde_json::from_str(&conteudo)?; // Converte o JSON de volta para struct

    loop {
        
        let agora = Local::now();
        if Local::now() >= lembrete.horario{
            notificacao(&lembrete.msg, lembrete.ativo, &lembrete.id); // Chamando a função
            break;
        } 
        println!("Aguardando... agora: {} | alarme: {}", agora.format("%H:%M:%S"), lembrete.horario.format("%H:%M:%S"));
        thread::sleep(StdDuration::from_secs(10)); // checa a cada 10 segundos  
    }
    println!("Encerrado.");

    Ok(())
}

// Função que dispara a notificação utilizando as informações providas pelos parâmetros
fn notificacao(msg: &String, ativo: bool, id: &i32) {
    if ativo == true {
        Notification::new()
            .summary("Lembrete")
            .body(msg)
            .show()
            .unwrap();
    } else {
        println!("ID do Lembrete: {id}")
    }
}
