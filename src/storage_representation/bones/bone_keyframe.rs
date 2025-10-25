use serde::{Serialize, Deserialize};
use crate::util::one;
use super::BoneTranslateKeyFrame;
use super::BoneRotateKeyFrame;
use super::BoneShearKeyFrame;
use super::BoneScaleKeyFrame;

#[derive(Serialize, Deserialize, Debug, Clone)]
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
