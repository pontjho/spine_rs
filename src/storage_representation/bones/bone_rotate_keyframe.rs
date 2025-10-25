use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all(serialize = "camelCase", deserialize = "snake_case"))]
pub struct BoneRotateKeyFrame
{
    #[serde(default)]
    pub time: f32,
    // #[serde(default, deserialize_with="deserialize_animation_interpolation")]
    // pub curve: AnimationInterpolation,
    #[serde(default)]
    #[serde(alias = "value")]
    pub angle: f32
}