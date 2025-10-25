use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all(serialize = "camelCase", deserialize = "snake_case"))]
pub struct PointAttachment
{
    pub x: f32,
    pub y: f32,
    pub rotation: f32
}
