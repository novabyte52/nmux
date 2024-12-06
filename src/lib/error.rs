use serde_yml;
use clap;

use crate::define_error;

define_error!(NmuxError,
    Clap(clap::Error),
    Serde(serde_yml::Error),
    Io(std::io::Error),
    Custom(String),
);
