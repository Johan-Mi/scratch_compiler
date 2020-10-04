use crate::target::*;
use serde::Serialize;
use std::collections::HashMap;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Stage {
    #[serde(flatten)]
    pub shared: SharedTarget,
}

impl Stage {
    pub fn new() -> Self {
        Self {
            shared: SharedTarget {
                is_stage: true,
                name: "Stage".to_string(),
                variables: HashMap::new(),
                lists: HashMap::new(),
                broadcasts: HashMap::new(),
                blocks: HashMap::new(),
                costumes: Vec::new(),
                sounds: Vec::new(),
                volume: 0,
                layer_order: 0,
            },
        }
    }
}

#[derive(Serialize)]
#[serde(rename_all = "kebab-case")]
enum VideoState {
    Off,
    On,
    OnFlipped,
}
