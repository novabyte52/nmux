use std::path::PathBuf;

use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct Pane {
    pub command: String,
    pub focus: bool,
    pub start_dir: PathBuf
}

#[derive(Debug, Serialize)]
pub struct Window {
    pub name: String,
    pub panes: Vec<Pane>
}

#[derive(Debug, Serialize)]
pub struct NmuxWorkflow {
    pub name: String,
    pub windows: Vec<Window>
}
