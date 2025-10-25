use serde::Deserialize;
use crate::util::one;
use super::BoneTranslateKeyFrame;
use super::BoneRotateKeyFrame;
use super::BoneShearKeyFrame;
use super::BoneScaleKeyFrame;

#[derive(Deserialize, Debug, Clone)]
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
