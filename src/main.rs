use clap::{Parser, Subcommand};
mod timer;
mod tracker;
mod blocker;

use blocker::BlockList;

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

}