use crate::monitor::*;
use crate::target::*;
use serde::Serialize;

#[derive(Serialize)]
pub struct Project {
    pub targets: Vec<Target>,
    monitors: Vec<Monitor>,
    meta: ProjectMetadata,
}

impl Project {
    pub fn new() -> Self {
        Self {
            targets: Vec::new(),
            monitors: Vec::new(),
            meta: ProjectMetadata::new(),
        }
    }
}

#[derive(Serialize)]
struct ProjectMetadata {
    semver: String,
    vm: String,
    agent: String,
}

impl ProjectMetadata {
    pub fn new() -> Self {
        Self {
            semver: "3.0.0".to_string(),
            vm: "0.0.0".to_string(),
            agent: "007".to_string(),
        }
    }
}
