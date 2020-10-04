mod asset;
mod block;
mod costume;
mod id;
mod monitor;
mod project;
mod sound;
mod sprite;
mod stage;
mod target;
use project::*;
use stage::*;
use target::*;

fn main() {
    let mut project = Project::new();
    project.targets.push(Target::Stage(Stage::new()));

    let serialized = serde_json::to_string_pretty(&project).unwrap();

    println!("{}", serialized);
}
