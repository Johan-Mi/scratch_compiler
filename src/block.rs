use crate::id::*;
use crate::match_variants;
use serde::ser::Serializer;
use serde::Serialize;
use std::boxed::Box;

#[derive(Serialize)]
#[serde(tag = "opcode")]
pub enum Block {
    #[serde(rename = "event_onflagclicked")]
    OnFlagClicked(OnFlagClicked),
}

impl Block {
    pub fn id(&self) -> &ID {
        match_variants!(self,
                        Block { OnFlagClicked },
                        b => &b.id)
    }
}

fn serialize_block_id<S>(
    block_box: &Option<Box<Block>>,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    block_box.as_ref().map(|b| b.id()).serialize(serializer)
}

#[derive(Serialize)]
pub struct OnFlagClicked {
    #[serde(skip)]
    id: ID,

    #[serde(serialize_with = "serialize_block_id")]
    next: Option<Box<Block>>,
}

impl OnFlagClicked {
    pub fn new(next: Option<Box<Block>>) -> Self {
        Self { id: new_id(), next }
    }
}
