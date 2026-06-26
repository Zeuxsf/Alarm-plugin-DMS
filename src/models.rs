use chrono::NaiveTime;
use notify_rust::Notification; 
use serde::{Serialize, Deserialize}; 

#[derive(Serialize, Deserialize)]
pub struct Alarme {
    pub mensagem: String,
    pub categoria: String, 
    pub a_delete: bool,
    pub horario: NaiveTime,
}

// trait Notificacao {
//     fn notificacao(&self);
// }

impl Alarme {
    pub fn notificacao(&self) {
        Notification::new()
            .summary(&self.categoria)
            .body(&self.mensagem)
            .show()
            .unwrap();
    }
}