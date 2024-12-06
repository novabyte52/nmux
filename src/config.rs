use std::path::PathBuf;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct NmuxConfig {
    pub workflows_path: PathBuf,
}

impl ::std::default::Default for NmuxConfig {
    fn default() -> Self {
        Self {
            workflows_path: PathBuf::from("$HOME/.nmux/workflows"),
        }
    }
}
