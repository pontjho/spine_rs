use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Bone
{

    pub name: String,
    pub length: f32,
    pub transform: BoneTransform,
    pub skin: bool,
    pub x: f32,
    pub y: f32,
    pub rotation: f32,
    pub scale_x: f32,
    pub scale_y: f32,
    pub shear_x: f32,
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
