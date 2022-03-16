use crate::animation_interpolation::AnimationInterpolation;
use crate::animation_interpolation::deserialize_animation_interpolation;
use serde::{Serialize, Deserialize};
use crate::util::one;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all(serialize = "camelCase", deserialize = "snake_case"))]
pub struct BoneKeyFrame
{
    #[serde(default)]
    pub rotate: Vec<BoneRotateKeyFrame>,
    #[serde(default)]
    pub translate: Vec<BoneTranslateKeyFrame>,
    #[serde(default)]
    pub scale: Vec<BoneScaleKeyFrame>,
    #[serde(default)]
    pub shear: Vec<BoneShearKeyFrame>
}

// pub enum BoneKeyFrameType
// {
//     Rotate,
//     Translate,
//     Scale,
//     Shear
// }

#[derive(Serialize, Deserialize, Debug)]
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

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all(serialize = "camelCase", deserialize = "snake_case"))]
pub struct BoneTranslateKeyFrame
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

#[derive(Serialize, Deserialize, Debug)]
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

#[derive(Serialize, Deserialize, Debug)]
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
