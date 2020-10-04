mod asset;
mod block;
mod costume;
mod id;
mod monitor;
mod primitive;
mod project;
mod sound;
mod sprite;
mod stage;
mod target;
use costume::*;
use project::*;
use stage::*;
use std::fs;
use target::*;

use primitive::*;

fn create_sb3(project: &Project) -> zip::result::ZipResult<()> {
    use std::io::Write;

    let project_json = serde_json::to_string_pretty(project).unwrap();
    println!("{}", project_json);

    let file = fs::File::create("project.sb3").unwrap();
    let mut writer = zip::ZipWriter::new(file);
    let options = zip::write::FileOptions::default();
    writer.start_file("project.json", options)?;
    writer.write(project_json.as_bytes())?;

    for t in &project.targets {
        for c in &match t {
            Target::Stage(s) => &s.shared,
            Target::Sprite(s) => &s.shared,
        }
        .costumes
        {
            writer.start_file(&c.shared.md5ext, options)?;
            writer.write(&fs::read(&c.shared.name).unwrap())?;
        }
    }

    Ok(())
}

fn main() {
    let mut project = Project::new();
    let mut stage = Stage::new();
    stage
        .shared
        .costumes
        .push(Costume::new("resources/empty_costume.svg".to_string()));
    stage.shared.costumes[0].resolve();
    project.targets.push(Target::Stage(stage));

    create_sb3(&project).unwrap();
}
