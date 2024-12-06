use std::{fs::File, process::Command};

use clap::{ Parser, Subcommand};
use dialoguer::{Confirm, Input};
use nmux_lib::{error::NmuxError, prompt_for_file_path, types::{NmuxWorkflow, Pane, Window}};

pub mod config;

#[derive(Parser)] // Derives command-line argument parsing for this struct
#[command(name = "nmux")]
#[command(about = "A CLI tool for managing tmuxp workflows", version = "1.0")]
struct Cli {
    #[command(subcommand)] // Specify that the CLI will use subcommands
    command: Option<Commands>,
    
    /// Append to the current session
    #[arg(short, long, default_value_t = false)]
    append: bool,
    
    /// The name of the workflow to open (optional when using subcommands)
    #[arg(required = false)]
    workflow_name: Option<String>,
}

#[derive(Subcommand)] // Defines the possible subcommands
enum Commands {
    /// Create a new workflow
    Create,
    Freeze
}

fn main() -> Result<(), NmuxError> {
    let cli = Cli::parse(); // Parse the CLI arguments

    let home = match homedir::my_home() {
        Ok(o) => match o {
            Some(o) => o,
            None => {
                eprintln!("Unable to locate home directory!");
                panic!();
            }
        },
        Err(e) => {
            eprintln!("Unable to get home directory! {}", e);
            panic!();
        }
    };
    
    match &cli.command {
        Some(Commands::Create) => {
            let workflow_name: String = Input::new().with_prompt("What do you want to name this workflow?").interact_text().unwrap();
            let mut windows: Vec<Window> = vec![];

            loop {
                let mut panes: Vec<Pane> = vec![];
                loop {
                    let command: String = Input::new().with_prompt("What command should be run ").interact_text().unwrap();
                    let focus = Confirm::new().with_prompt("Do you want to focus this pane?").interact().unwrap();
                    let start_dir = prompt_for_file_path();

                    let pane = Pane {
                        command,
                        focus,
                        start_dir
                    };
                    
                    panes.push(pane);

                    let make_another_pane = Confirm::new().with_prompt("Would you like to add another window?").interact().unwrap();
                    if !make_another_pane {break;}
                }

                let window_name = Input::new().with_prompt("What do you want to name this window?").interact_text().unwrap();

                let window = Window {
                    name: window_name,
                    panes
                };

                windows.push(window);
                
                let make_another_window = Confirm::new().with_prompt("Would you like to add another window?").interact().unwrap();
                if !make_another_window {break;}
            }

            let workflow = NmuxWorkflow {
                name: workflow_name.clone(),
                windows
            };

            let workflow_file = File::create(format!("{}.yaml", workflow_name)).unwrap();

            match serde_yml::to_writer(workflow_file, &workflow) {
                Ok(()) => return Ok(()),
                Err(e) => return Err(NmuxError::Serde(e))
            };
        }

        Some(Commands::Freeze) => {
            let mut tmuxp = Command::new("tmuxp");
            tmuxp.current_dir(home).arg("freeze");
            match tmuxp.output() {
                Ok(o) => {
                    println!("tmuxp output: {:#?}", o.stdout);
                    return Ok(());
                },
                Err(e) => return Err(NmuxError::Io(e))
            }
        }

        None => {
            let mut tmuxp = Command::new("tmuxp");
            if let Some(workflow_name) = cli.workflow_name {
                tmuxp
                    .current_dir(home)
                    .arg("load")
                    .arg(workflow_name);
                
                if cli.append {
                    tmuxp.arg("-a");
                }

                match tmuxp.output() {
                    Ok(o) => {
                        println!("tmuxp output: {:#?}", o.stdout);
                        return Ok(())
                    },
                    Err(e) => return Err(NmuxError::Io(e))
                }                
            } else {
                return Err(NmuxError::Custom("No workflow name passed".into()));
            }
        }
    }
}
