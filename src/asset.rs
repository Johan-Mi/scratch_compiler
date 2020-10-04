use crate::costume::*;
use crate::sound::*;
use serde::Serialize;

pub enum Asset {
    Costume(Costume),
    Sound(Sound),
}

#[derive(Serialize)]
pub struct SharedAsset {
    asset_id: String,
    name: String,
    md5ext: String,
    data_format: DataFormat,
}

#[derive(Serialize)]
enum DataFormat {
    Png,
    Svg,
    Jpeg,
    Jpg,
    Bmp,
    Gif,
    Wav,
    Wave,
    Mp3,
}
