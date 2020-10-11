use crate::block::*;
use crate::id::*;

#[derive(SimpleHatBlock)]
pub struct WhenFlagClicked {
    pub id: ID,
    pub next: Option<Box<Block>>,
}

impl WhenFlagClicked {
    const OPCODE: &'static str = "event_whenflagclicked";

    pub fn new(next: Option<Box<Block>>) -> Self {
        Self { id: new_id(), next }
    }
}

#[derive(SimpleHatBlock)]
pub struct WhenThisSpriteClicked {
    pub id: ID,
    pub next: Option<Box<Block>>,
}

impl WhenThisSpriteClicked {
    const OPCODE: &'static str = "event_whenthisspriteclicked";

    pub fn new(next: Option<Box<Block>>) -> Self {
        Self { id: new_id(), next }
    }
}
