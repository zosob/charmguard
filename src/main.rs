use clap::{Parser, Subcommand};
mod timer;
mod tracker;
mod blocker;
mod session;
mod activity;

use blocker::BlockList;
use session::SessionMetrics;
use chrono::Local;
use std::{thread, time::Duration};
use activity::get_active_window_title;

#[derive(Parser)]
#[command(version, about = "Sip tea. Block distractions. Stay focused")]

struct Cli{
    //Length of focus sessions...
    #[arg(short, long, default_value_t = 25)]
    focus: u32,

    //Path to block-list YAML
    #[arg(short, long, default_value="data/distractions.yaml")]
    blocklist: String,

    //Enable logging of active windows
    #[arg(long)]
    track:bool,

    //Run post-session analysis
    #[arg(long)]
    analyze: bool,

    //Sub-commands
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands{
    Report, //Productivity report...
}

fn main(){
    let args = Cli::parse();

    println!("CharmGuard starting - {}-minute focus session", args.focus);
    //Focus timer...
    if args.focus>0 {
        timer::start(args.focus);
    }

    let bl = BlockList::load(&args.blocklist);
    let snaps = tracker::capture_snapshot();
    let mut distracted=0;
    let mut switches = 0;
    let mut last_title = String::new();
    let mut distract_hits = 0;
    
    for s in &snaps {
        if bl.is_distracting(&s.name){
            distracted+=1;
            println!("DISTRACTION: {}  (CPU {:1}%, Mem {} KB", s.name, s.cpu, s.memory);
        }
    }

    println!("Tracked: {}, Distracting: {}", snaps.len(), distracted);

    if args.track{
        println!("Capturing process snapshot...");
        let snaps = tracker::capture_snapshot();
        for entry in snaps.iter().take(5){
            println!("~[{}] {} | CPU: {:.1} % | Mem: {} KB",
                entry.timestamp, entry.name, entry.cpu, entry.memory);
        }
        println!("Total Processes tracked: {}", snaps.len());
    }


    for _ in 0..(args.focus * 60){
        if let Some(title) = get_active_window_title(){
            if title != last_title && !title.is_empty(){
                switches += 1;
                println!("Switched to: {}", title);
                last_title = title.clone();

                //Basic distraction check
                if bl.is_distracting(&title){
                    distract_hits += 1;
                    println!("Distracting window detected ({distract_hits} total)");
                }
            }
        }
        thread::sleep(Duration::from_secs(1));
    }

    print!("Focus session complete!\n");

    let metrics = SessionMetrics{
        start: Local::now() - chrono::Duration::minutes(args.focus as i64),
        duration_min: args.focus as u32,
        window_switches: switches,
        distractor_hits: distracted,
        total_processes: snaps.len() as u32,
        idle_seconds: 0,
    };
    std::fs::create_dir_all("output").ok();
    metrics.save_csv("output/sessions.csv");
    println!("Session saved for ML Training.");

}