use cgmath::Deg;

use crate::{animation_utilities::interpolation::Interpolatable, storage_representation::bones::BoneRotateKeyFrame};

impl Interpolatable for BoneRotateKeyFrame
{
    type Value = Deg<f32>;

    fn time(&self) -> f32
    {
        self.time
    }

    fn get_value(&self) -> Self::Value
    {
        cgmath::Deg(self.angle).into()
    }
}
