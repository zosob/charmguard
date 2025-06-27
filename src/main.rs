use clap::{Parser, Subcommand};
use chrono::Local;
use std::{thread, time::Duration};

mod timer;
mod tracker;
mod blocker;
mod session;
mod activity;

use blocker::BlockList;
use session::SessionMetrics;
use activity::get_active_window_title;

#[derive(Parser)]
#[command(
    name = "CharmGuard",
    version = "0.3", 
    about = "Sip tea. Block distractions. Stay focused"
)]

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

    //Close distracting windows when drift is detected
    #[arg(long)]
    autoblock: bool,

    //Custom drift score threshold -0.2
    #[arg(long, default_value_t = -0.2)]
    drift_threshold: f32,
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

    //-----Loading the block list-----
    let bl = BlockList::load(&args.blocklist);
    
    //-----Initial process snapshot-----
    let snaps = tracker::capture_snapshot();
    let mut switches = 0;
    let mut last_title = String::new();
    let mut distracted_hits = 0;
    
    for s in &snaps {
        if bl.is_distracting(&s.name){
            distracted_hits+=1;
            println!("DISTRACTION: {}  (CPU {:1}%, Mem {} KB", s.name, s.cpu, s.memory);
        }
    }

    println!("Tracked: {}, Distracting: {}", snaps.len(), distracted_hits);

    //-----Quick display-----
    if args.track{
        println!("Capturing process snapshot...");
        for entry in snaps.iter().take(5){
            println!("~[{}] {} | CPU: {:.1} % | Mem: {} KB",
                entry.timestamp, entry.name, entry.cpu, entry.memory);
        }
        println!("Total Processes tracked: {}", snaps.len());
    }

    //-----Do a live window tracking-----
    for second in 0..(args.focus * 60){
        if let Some(title) = get_active_window_title(){
            if title != last_title && !title.is_empty(){
                switches += 1;
                println!("Switched to: {}", title);
                
                //-----Basic distraction check-----
                if bl.is_distracting(&title){
                    distracted_hits += 1;
                    println!("Distracting window detected ({distracted_hits} total)");
                }
                last_title = title.clone();
            }
        }
        if second % 10 == 0 {
            let live_metrics = SessionMetrics{
                start: Local::now(),
                duration_min: args.focus,
                window_switches: switches,
                distractor_hits: distracted_hits,
                total_processes: snaps.len() as u32,
                idle_seconds: 0,
            };
            live_metrics.save_csv("output/last_session.csv");
            let out = std::process::Command::new("python")
                .arg("py/predict_drift.py")
                .arg("output/last_session.csv")
                .output()
                .expect("Failed to run predictor");

            let output_str = String::from_utf8_lossy(&out.stdout);
            let score: f32 = output_str
                .trim()
                .split(',')
                .nth(1)
                .and_then(|s| s.parse().ok())
                .unwrap_or(0.0);

            if score < args.drift_threshold {
                println!("Drift Detected! Focus leaking like a teabag in rain... (score: {score:.2})");

                if args.autoblock && !last_title.is_empty(){
                    println!("🤚Auto-blocking: {}",last_title);
                    activity::close_window_by_title(&last_title);
            }
        }
    }
        thread::sleep(Duration::from_secs(1));
    }

    print!("\n👌Focus session complete!\n");

    //-----Save session metrics-----
    let metrics = SessionMetrics{
        start: Local::now() - chrono::Duration::minutes(args.focus as i64),
        duration_min: args.focus as u32,
        window_switches: switches,
        distractor_hits: distracted_hits,
        total_processes: snaps.len() as u32,
        idle_seconds: 0,
    };
    std::fs::create_dir_all("output").ok();
    metrics.save_csv("output/sessions.csv");
    metrics.save_csv("output/last_sessions.csv");
    println!("📈Session saved for ML Training.");

    //-----ML Focus drift analysis-----
    if args.analyze{
        //Write last-session row to temp file
        println!("🔎Running ML Drift ananlysis...");
        metrics.save_csv("output/last_session.csv");

        let out = std::process::Command::new("python")
            .arg("py/predict_drift.py")
            .arg("output/last_session.csv")
            .output()
            .expect("Failed to run predictor");

        if out.status.success(){
            let res = String::from_utf8_lossy(&out.stdout);
            let parts: Vec<&str> = res.trim().split(',').collect();
            if parts.len() == 2 {
                let label = parts[0];
                let score: f32 = parts[1].parse().unwrap_or(0.0);
                if label == "-1" {
                    println!("⚠️⚠️Focus drift detected! (score {score:.3})");
                } else {
                    println!("🍵🍵All clear! Focus steady! (score {score:.3})");
                }
            } else {
                eprintln!("Python error: \n{}", String::from_utf8_lossy(&out.stderr));
            }
        }            
    }
}