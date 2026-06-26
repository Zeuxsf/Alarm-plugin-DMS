use crate::models::Alarme;
use crate::utils;

pub fn menu() {
    loop {
        println!(r#"
1 - Criar Alarme
3 - Disparar Alarme
2 - Sair
        "#);

        let opt = utils::input_menu();

        match opt {
            1 => criar_alarme(),
            2 => break,            
            3 => break,
            _ => println!("Invalid"),
        }
    }

}

fn criar_alarme() {
    
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

    println!("Alarme: {} Hora salva: {}", alarme.mensagem, alarme.horario);
    alarme.notificacao();
}

fn disparar_notificacao() {

}
