use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all(serialize = "camelCase", deserialize = "snake_case"))]
pub struct Skeleton
{
    pub hash: String,
    pub version: String,
    #[serde(default)]
    pub x: f32,
    #[serde(default)]
    pub y: f32,
    pub width: f32,
    pub height: f32
}
