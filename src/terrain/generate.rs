use bevy::{prelude::Component, tasks::Task};
use std::thread::sleep;

// Attribution: https://stackoverflow.com/questions/71504675/how-to-make-async-system-function-in-the-bevy-game-engine
#[derive(Component)]
pub struct GenerateTask {
    pub task: Task<Vec<i32>>,
}

pub fn calculate_trigger() {
    println!("starting calculation...");
    sleep(std::time::Duration::from_secs(2));
    println!("calculate complete!");
}

pub fn calculate() {
    println!("starting calculation...");
    sleep(std::time::Duration::from_secs(2));
    println!("calculate complete!");
}
