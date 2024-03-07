use thiserror::Error;

pub mod scheduling_problem;
pub mod settings;

pub const WEEKDAYS: [&str; 5] = ["Monday", "Tuesday", "Wednesday", "Thursday", "Friday"];

#[derive(Error, Debug)]
pub enum SchedularError {
    #[error("Failed to add vairiable {0}")]
    VariableAddingError(#[from] grb::Error),
}
