use serde::{Serialize, Deserialize};
use crate::util::one;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all(serialize = "camelCase", deserialize = "snake_case"))]
pub struct Bone
{
    pub name: String,
    
    pub parent: Option<String>,

    #[serde(default)]
    pub length: f32,

    #[serde(default)]
    pub transform: BoneTransform,

    #[serde(default)]
    pub skin: bool,

    #[serde(default)]
    pub x: f32,

    #[serde(default)]
    pub y: f32,

    #[serde(default)]
    pub rotation: f32,

    #[serde(default="one")]
    #[serde(alias = "scaleX")]
    pub scale_x: f32,

    #[serde(default="one")]
    #[serde(alias = "scaleY")]
    pub scale_y: f32,

    #[serde(default)]
    pub shear_x: f32,

    #[serde(default)]
    pub shear_y: f32
}

#[derive(Serialize, Deserialize, Debug)]
pub enum BoneTransform
{
    Normal,
    OnlyTranslation,
    NoRotationOrReflection,
    NoScale,
    NoScaleOrReflection
}

impl Default for BoneTransform
{
    fn default() -> Self
    {
        BoneTransform::Normal
    }
}
