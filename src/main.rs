use clap::{Parser, Subcommand};
mod timer;

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
}