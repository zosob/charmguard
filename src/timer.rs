use std::{thread, time::Duration};
use chrono::Local;

pub fn start(minutes: u32){
    println!("Starting focus timer for {} minutes - good luck!", minutes);

    let total_secs = minutes*60;

    for remaining in (0..=total_secs).rev(){
        let mins = remaining / 60;
        let secs = remaining % 60;

        println!("\r {:02}:{:02} remaining", mins, secs);
        std::io::Write::flush(&mut std::io::stdout()).unwrap();
        thread::sleep(Duration::from_secs(1));
    }
    let end = Local::now().format("%I:%M:%S %p");
    println!("\n Focus session complete at {end} - well done!");
}