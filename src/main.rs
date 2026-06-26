// use chrono::{DateTime, Local, Duration, NaiveTime};
// use notify_rust::Notification; 
// use serde::{Serialize, Deserialize}; 
// use std::fs;
// use std::thread;
// use std::time::Duration as StdDuration;

mod utils;
mod menu;
mod models;


fn main() -> Result<(), Box<dyn std::error::Error>> {

    menu::menu();

    Ok(())
}
