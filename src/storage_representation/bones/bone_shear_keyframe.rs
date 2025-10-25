use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all(serialize = "camelCase", deserialize = "snake_case"))]
pub struct BoneShearKeyFrame
{
    #[serde(default)]
    pub time: f32,
    // #[serde(default, deserialize_with="deserialize_animation_interpolation")]
    // pub curve: AnimationInterpolation,
    #[serde(default)]
    pub x: f32,
    #[serde(default)]
    pub y: f32
}
