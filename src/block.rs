use crate::block_types::*;
use crate::id::*;
use crate::match_variants;
use serde::ser::{SerializeMap, Serializer};
use serde::Serialize;

pub enum Block {
    WhenFlagClicked(WhenFlagClicked),
    WhenThisSpriteClicked(WhenThisSpriteClicked),
}

impl Block {
    pub fn id(&self) -> &ID {
        match_variants!(self,
                        Block { WhenFlagClicked, WhenThisSpriteClicked },
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
                        Block { WhenFlagClicked, WhenThisSpriteClicked },
                        b => b.serialize_with_parent(parent, serializer))
    }
}

pub fn serialize_block_slice<S>(
    blocks: &[Block],
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

pub type BlockIterStack<'a> = Vec<(&'a Block, Option<&'a ID>)>;

pub struct BlockIter<'a> {
    stack: BlockIterStack<'a>,
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
        match_variants!(retval.0,
                        Block { WhenFlagClicked, WhenThisSpriteClicked },
                        b => b.push_next(&mut self.stack));
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

pub trait Opcode {
    const OPCODE: &'static str;
}

pub trait SerializeNext {
    fn serialize_next<S>(&self, _obj: &mut S) -> Result<(), S::Error>
    where
        S: SerializeMap,
    {
        Ok(())
    }
}

pub trait SerializableBlock: Opcode + SerializeNext {
    fn serialize_with_parent<S>(
        &self,
        parent: Option<&ID>,
        serializer: S,
    ) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut obj = serializer.serialize_map(None)?;

        obj.serialize_entry("opcode", Self::OPCODE)?;
        self.serialize_parent(&mut obj, parent)?;
        self.serialize_next(&mut obj)?;

        obj.end()
    }

    fn serialize_parent<S>(
        &self,
        obj: &mut S,
        parent: Option<&ID>,
    ) -> Result<(), S::Error>
    where
        S: SerializeMap,
    {
        obj.serialize_entry("parent", &parent)?;
        if parent.is_none() {
            obj.serialize_entry("topLevel", &true)?;
            obj.serialize_entry("x", &0)?;
            obj.serialize_entry("y", &0)?;
        }
        Ok(())
    }
}
