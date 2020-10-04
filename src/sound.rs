use crate::asset::*;
use serde::Serialize;

#[derive(Serialize)]
pub struct Sound {
    #[serde(flatten)]
    shared: SharedAsset,

    rate: u32,
    sample_count: u32,
}
