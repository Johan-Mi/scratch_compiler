use crate::sprite::*;
use crate::stage::*;
use serde::Serialize;

#[derive(Serialize)]
#[serde(untagged)]
pub enum Target {
    Stage(Stage),
    Sprite(Sprite),
}
