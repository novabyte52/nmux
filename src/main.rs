use std::{path::PathBuf, process::Command};

use clap::{builder::Str, Parser, Subcommand};

pub mod config;

#[derive(Clone, Debug, PartialEq)]
pub struct Pane {
    command: String,
    focus: bool,
    start_dir: PathBuf
}

#[derive(Clone, Debug, PartialEq)]
pub struct Window {
    name: String,
    panes: Vec<Pane>
}

#[derive(Subcommand, Clone, Debug, PartialEq)]
pub enum SubCommands {
    Create { workflow: String, session_name: String, windows: Vec<Window> },
    Delete { workflow: String, session_name: String, },
    None,
}

#[derive(Debug, Parser)]
#[command(version, about)]
struct NmuxArgs {
    #[command(subcommand)]
    cmd: SubCommands,

    workflow: Option<String>,

    #[arg(short, long)]
    create: bool,

    #[arg(short, long)]
    append: bool,

    #[arg(short, long)]
    close: bool,

    #[arg(short, long)]
    workflows: Vec<String>,
}

fn main() {
    let home = match homedir::my_home() {
        Ok(o) => match o {
            Some(o) => o,
            None => {
                eprintln!("Unable to locate home directory!");
                return;
            }
        },
        Err(e) => {
            eprintln!("Unable to get home directory! {}", e);
            return;
        }
    };
    
    let mut tmuxp = Command::new("tmuxp");
    let args = NmuxArgs::parse();
    println!("args passed: {:#?}", args);

    match args.cmd {
        SubCommands::Create(w) => { tmuxp.current_dir(home).arg() workflow }
    }


    tmuxp
        .current_dir(home)
        .arg("load")
        .arg(args.workflow.expect("No workflow provided to open!"));

    if args.append {
        tmuxp.arg("-a");
    }

    let out = tmuxp.output();

    println!("{:#?}", out);
}
