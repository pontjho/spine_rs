use serde::Deserialize;
use crate::util::one;

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all(serialize = "camelCase", deserialize = "snake_case"))]
pub struct SlotColourKeyFrame
{
    #[serde(default)]
    pub time: f32,
    // #[serde(default, deserialize_with="deserialize_animation_interpolation")]
    // pub curve: AnimationInterpolation,
    #[serde(default)]
    pub c2: f32,
    #[serde(default="one")]
    pub c3: f32,
    #[serde(default="one")]
    pub c4: f32
}
