use crate::animation_interpolation::AnimationInterpolation;

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

pub struct BoneRotateKeyFrame
{
    pub time: f32,
    pub curve: AnimationInterpolation,
    pub angle: f32
}

pub struct BoneTranslateKeyFrame
{
    pub time: f32,
    pub curve: AnimationInterpolation,
    pub x: f32,
    pub y: f32
}

pub struct BoneScaleKeyFrame
{
    pub time: f32,
    pub curve: AnimationInterpolation,
    pub x: f32,
    pub y: f32
}

pub struct BoneShearKeyFrame
{
    pub time: f32,
    pub curve: AnimationInterpolation,
    pub x: f32,
    pub y: f32
}
