use crate::id::*;
use serde::ser::{SerializeTuple, Serializer};
use serde::Serialize;

#[derive(Serialize)]
#[serde(untagged)]
pub enum Primitive {
    NumPrimitive(NumPrimitive),
    TextPrimitive(TextPrimitive),
    BroadcastPrimitive(BroadcastPrimitive),
    VariablePrimitive(VariablePrimitive),
    ListPrimitive(ListPrimitive),
}

pub struct NumPrimitive {
    pub value: f32,
}

impl Serialize for NumPrimitive {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut tup = serializer.serialize_tuple(2)?;
        tup.serialize_element(&4)?;
        tup.serialize_element(&self.value)?;
        tup.end()
    }
}

pub struct TextPrimitive {
    pub text: String,
}

impl Serialize for TextPrimitive {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut tup = serializer.serialize_tuple(2)?;
        tup.serialize_element(&10)?;
        tup.serialize_element(&self.text)?;
        tup.end()
    }
}

pub struct BroadcastPrimitive {
    pub message_name: String,
    pub message_id: ID,
}

impl Serialize for BroadcastPrimitive {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut tup = serializer.serialize_tuple(3)?;
        tup.serialize_element(&11)?;
        tup.serialize_element(&self.message_name)?;
        tup.serialize_element(&self.message_id)?;
        tup.end()
    }
}

pub struct VariablePrimitive {
    pub var_name: String,
    pub var_id: ID,
    pub x: Option<f32>,
    pub y: Option<f32>,
}

impl Serialize for VariablePrimitive {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match (self.x, self.y) {
            (Some(x), Some(y)) => {
                let mut tup = serializer.serialize_tuple(5)?;
                tup.serialize_element(&12)?;
                tup.serialize_element(&self.var_name)?;
                tup.serialize_element(&self.var_id)?;
                tup.serialize_element(&x)?;
                tup.serialize_element(&y)?;
                tup.end()
            }
            _ => {
                let mut tup = serializer.serialize_tuple(3)?;
                tup.serialize_element(&12)?;
                tup.serialize_element(&self.var_name)?;
                tup.serialize_element(&self.var_id)?;
                tup.end()
            }
        }
    }
}

pub struct ListPrimitive {
    pub list_name: String,
    pub list_id: ID,
    pub x: Option<f32>,
    pub y: Option<f32>,
}

impl Serialize for ListPrimitive {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match (self.x, self.y) {
            (Some(x), Some(y)) => {
                let mut tup = serializer.serialize_tuple(5)?;
                tup.serialize_element(&12)?;
                tup.serialize_element(&self.list_name)?;
                tup.serialize_element(&self.list_id)?;
                tup.serialize_element(&x)?;
                tup.serialize_element(&y)?;
                tup.end()
            }
            _ => {
                let mut tup = serializer.serialize_tuple(3)?;
                tup.serialize_element(&12)?;
                tup.serialize_element(&self.list_name)?;
                tup.serialize_element(&self.list_id)?;
                tup.end()
            }
        }
    }
}
