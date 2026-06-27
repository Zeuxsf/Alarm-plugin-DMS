use crate::models::Alarme;
use crate::utils;
use std::fs;


pub fn menu() {
    loop {
        println!(r#"
1 - Criar Alarme
2 - Disparar Alarme
3 - Sair
        "#);

        let opt = utils::input_menu();

        match opt {
            1 => criar_alarme(),
            2 => disparar_notificacao().expect("ok"),            
            3 => break,
            _ => println!("Invalid"),
        }
    }

}

fn criar_alarme() {

    let mut alarmes: Vec<Alarme> = match fs::read_to_string("alarmes.json") {
        Ok(conteudo) => serde_json::from_str(&conteudo).unwrap_or(Vec::new()),
        Err(_) => Vec::new(),
    };

    println!("Digite a categoria do alarme: ");
    let categoria = utils::input();

    println!("Digite a mensagem do alarme: ");
    let mensagem = utils::input();


    let horario = match utils::input_time() {
        Some(time) => time,
        None => return,
    };
    
    let alarme: Alarme = Alarme {
        mensagem,
        categoria,
        a_delete: true,
        horario
    };
    
    alarmes.push(alarme);

    let json = serde_json::to_string_pretty(&alarmes).unwrap();
    fs::write("alarmes.json", &json).unwrap();

}

fn disparar_notificacao() -> Result<(), Box<dyn std::error::Error>> {
    
    match fs::exists("alarmes.json") {
        Ok(true) => println!("Alarmes: "),
        Ok(false) => { 
            println!("O arquivo não existe.");
            return Ok(());
        }
        Err(e) => return Err(e.into())
    }

    let json = fs::read_to_string("alarmes.json")?;
    let alarmes: Vec<Alarme> = serde_json::from_str(&json)?;

    let mut count: i8 = 0; 
    for alarme in &alarmes {
        
        print!("{count} - {}", alarme.mensagem);
        count += 1;
    }

    println!("Qual alarme deseja utilizar?");
    let input = utils::input_menu();

    if input as usize >= alarmes.len() {
        println!("Esse alarme não existe.");
        return Ok(());
    }

    alarmes[input as usize].notificacao();

    //alarme.notificacao();
    Ok(())
}
