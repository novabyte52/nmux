use std::path::PathBuf;

use dialoguer::Input;

pub mod types;
pub mod define_error;
pub mod error;

pub fn prompt_for_file_path() -> PathBuf {
    Input::new()
        .with_prompt("Enter the file path")
        .validate_with(|input: &String| {
            if PathBuf::from(input).exists() {
                Ok(())
            } else {
                Err("File path does not exist")
            }
        })
        .interact_text()
        .unwrap().parse().unwrap()
}
