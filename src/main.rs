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
use block::*;
use costume::*;
use project::*;
use stage::*;
use std::fs;
use target::*;

fn create_sb3(project: &Project) -> zip::result::ZipResult<()> {
    use std::io::Write;

    let project_json = serde_json::to_string_pretty(project).unwrap();
    println!("{}", project_json);

    let file = fs::File::create("project.sb3")?;
    let mut writer = zip::ZipWriter::new(file);
    let options = zip::write::FileOptions::default();
    writer.start_file("project.json", options)?;
    writer.write_all(project_json.as_bytes())?;

    for t in &project.targets {
        for c in match t {
            Target::Stage(s) => &s.costumes,
            Target::Sprite(s) => &s.costumes,
        } {
            writer.start_file(&c.shared.md5ext, options)?;
            let file_content = fs::read(&c.shared.name)?;
            writer.write_all(&file_content)?;
        }
    }

    Ok(())
}

fn main() {
    let mut project = Project::new();
    let mut stage = Stage::new();

    let mut b = Block::new();
    b.top_level = true;
    b.x = Some(0);
    b.y = Some(0);
    b.opcode = "event_whenflagclicked".to_string();
    stage.blocks.push(b);

    stage
        .costumes
        .push(Costume::new("resources/empty_costume.svg".to_string()));
    stage.costumes[0].resolve();

    project.targets.push(Target::Stage(stage));

    create_sb3(&project).unwrap();
}
