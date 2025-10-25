

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
