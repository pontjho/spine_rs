use serde::{Deserialize};
use crate::util::one;
use crate::util::deserialize_colour;
use crate::util::ffffffff;

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all(serialize = "camelCase", deserialize = "snake_case"))]
pub struct RegionAttachment
{
    pub path: Option<String>,

    #[serde(default)]
    pub x: f32,

    #[serde(default)]
    pub y: f32,

    #[serde(default="one")]
    #[serde(alias = "scaleX")]
    pub scale_x: f32,

    #[serde(default="one")]
    #[serde(alias = "scaleY")]
    pub scale_y: f32,

    #[serde(default)]
    pub rotation: f32,
    pub width: f32,
    pub height: f32,

    #[serde(default="ffffffff", deserialize_with="deserialize_colour")]
    pub color: u32
}