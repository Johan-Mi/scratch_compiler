use crate::id::*;
use crate::match_variants;
use serde::ser::{SerializeMap, SerializeStruct, Serializer};
use serde::Serialize;
use std::boxed::Box;

pub enum Block {
    WhenFlagClicked(WhenFlagClicked),
}

impl Block {
    pub fn id(&self) -> &ID {
        match_variants!(self,
                        Block { WhenFlagClicked },
                        b => &b.id)
    }

    pub fn iter(&self) -> BlockIter {
        BlockIter::new(self)
    }

    pub fn serialize_with_parent<S>(
        &self,
        parent: Option<&ID>,
        serializer: S,
    ) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match_variants!(self,
                        Block { WhenFlagClicked },
                        b => b.serialize_with_parent(parent, serializer))
    }
}

pub fn serialize_block_vec<S>(
    blocks: &Vec<Block>,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let mut map = serializer.serialize_map(None)?;

    for (b, p) in blocks.iter().map(Block::iter).flatten() {
        map.serialize_key(b.id())?;
        map.serialize_value(&BlockAndParent {
            block: b,
            parent: p,
        })?;
    }

    map.end()
}

pub struct BlockIter<'a> {
    stack: Vec<(&'a Block, Option<&'a ID>)>,
}

impl<'a> BlockIter<'a> {
    pub fn new(first_block: &'a Block) -> Self {
        Self {
            stack: vec![(first_block, None)],
        }
    }
}

impl<'a> Iterator for BlockIter<'a> {
    type Item = (&'a Block, Option<&'a ID>);

    fn next(&mut self) -> Option<Self::Item> {
        let retval = self.stack.pop()?;

        match retval.0 {
            Block::WhenFlagClicked(b) => {
                if let Some(n) = &b.next {
                    self.stack.push((&n, Some(&b.id)));
                }
            }
        }

        Some(retval)
    }
}

pub struct BlockAndParent<'a> {
    pub block: &'a Block,
    pub parent: Option<&'a ID>,
}

impl<'a> Serialize for BlockAndParent<'a> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.block.serialize_with_parent(self.parent, serializer)
    }
}

pub struct WhenFlagClicked {
    id: ID,
    next: Option<Box<Block>>,
}

impl WhenFlagClicked {
    pub fn new(next: Option<Box<Block>>) -> Self {
        Self { id: new_id(), next }
    }

    pub fn serialize_with_parent<S>(
        &self,
        parent: Option<&ID>,
        serializer: S,
    ) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let field_count = if parent.is_none() { 6 } else { 3 };
        let mut obj =
            serializer.serialize_struct("WhenFlagClicked", field_count)?;

        obj.serialize_field("opcode", "event_whenflagclicked")?;
        obj.serialize_field("parent", &parent)?;
        obj.serialize_field("next", &self.next.as_ref().map(|b| b.id()))?;
        if parent.is_none() {
            obj.serialize_field("topLevel", &true)?;
            obj.serialize_field("x", &0)?;
            obj.serialize_field("y", &0)?;
        }

        obj.end()
    }
}
