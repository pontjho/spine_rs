use serde::{Deserialize, Serialize};
use crate::util::one;

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all(serialize = "camelCase", deserialize = "snake_case"))]
pub struct BoneScaleKeyFrame
{
    #[serde(default)]
    pub time: f32,
    // #[serde(default, deserialize_with="deserialize_animation_interpolation")]
    // pub curve: AnimationInterpolation,
    #[serde(default="one")]
    pub x: f32,
    #[serde(default="one")]
    pub y: f32
}
