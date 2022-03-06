use crate::animation_interpolation::AnimationInterpolation;

use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all(serialize = "camelCase", deserialize = "snake_case"))]
pub struct BoneKeyFrame
{
    pub rotate: Option<BoneRotateKeyFrame>,
    pub translate: Option<BoneTranslateKeyFrame>,
    pub scale: Option<BoneScaleKeyFrame>,
    pub shear: Option<BoneShearKeyFrame>
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
    pub time: f32,
    pub curve: AnimationInterpolation,
    pub angle: f32
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all(serialize = "camelCase", deserialize = "snake_case"))]
pub struct BoneTranslateKeyFrame
{
    pub time: f32,
    pub curve: AnimationInterpolation,
    pub x: f32,
    pub y: f32
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all(serialize = "camelCase", deserialize = "snake_case"))]
pub struct BoneScaleKeyFrame
{
    pub time: f32,
    pub curve: AnimationInterpolation,
    pub x: f32,
    pub y: f32
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all(serialize = "camelCase", deserialize = "snake_case"))]
pub struct BoneShearKeyFrame
{
    pub time: f32,
    pub curve: AnimationInterpolation,
    pub x: f32,
    pub y: f32
}
