use cgmath::Vector3;

use crate::{animation_utilities::interpolation::Interpolatable, storage_representation::bones::BoneTranslateKeyFrame};

impl Interpolatable for BoneTranslateKeyFrame
{
    type Value = Vector3<f32>;

    fn time(&self) -> f32
    {
        self.time
    }

    fn get_value(&self) -> Self::Value
    {
        self.get_translation()
    }
}
