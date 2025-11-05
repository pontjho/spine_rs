use cgmath::Vector2;
use cgmath::Matrix4;
use cgmath::Deg;
use cgmath::Vector3;

use crate::storage_representation::attachments::RegionAttachment;
use crate::storage_representation::bones::Bone;
use crate::storage_representation::bones::BoneRotateKeyFrame;
use crate::storage_representation::bones::BoneScaleKeyFrame;
use crate::storage_representation::bones::BoneShearKeyFrame;
use crate::storage_representation::bones::BoneTranslateKeyFrame;

pub trait CGMathIntegrations
{
    fn get_translation(&self) -> Vector3<f32>;
    fn get_rotation(&self) -> Deg<f32>;
    fn get_scale(&self) -> Vector2<f32>;
    fn get_shear(&self) -> Vector2<f32>;
}

pub trait InterpolationLogic
{
    fn interpolate_translation(&self, time: f32, from: BoneTranslateKeyFrame, to: BoneTranslateKeyFrame) -> (f32, f32);
    fn interpolate_scale(&self, time: f32, from: BoneScaleKeyFrame, to: BoneScaleKeyFrame) -> (f32, f32);
    fn interpolate_shear(&self, time: f32, from: BoneShearKeyFrame, to: BoneShearKeyFrame) -> (f32, f32);
    fn interpolate_rotation(&self, time: f32, from: BoneRotateKeyFrame, to: BoneRotateKeyFrame) -> f32;
}

impl CGMathIntegrations for Bone
{
    fn get_translation(&self) -> Vector3<f32>
    {
        Vector3::new(self.x, self.y, 0.0)
    }

    fn get_rotation(&self) -> Deg<f32>
    {
        cgmath::Deg(self.rotation)
    }

    fn get_scale(&self) -> Vector2<f32>
    {
        Vector2::new(self.scale_x, self.scale_y)
    }

    fn get_shear(&self) -> Vector2<f32>
    {
        Vector2::new(1., 1.)
    }
}

impl CGMathIntegrations for RegionAttachment
{
    fn get_translation(&self) -> Vector3<f32>
    {
        Vector3::new(self.x, self.y, 0.0)
    }

    fn get_rotation(&self) -> Deg<f32>
    {
        cgmath::Deg(self.rotation).into()
    }

    fn get_scale(&self) -> Vector2<f32>
    {
        Vector2::new(self.scale_x, self.scale_y)
    }

    fn get_shear(&self) -> Vector2<f32>
    {
        Vector2::new(1., 1.)
    }
}

impl RegionAttachment
{
    pub fn get_transform(&self, base_transform: &Matrix4<f32>, _attachment_name: &str) -> Matrix4<f32>
    {
        let scale = self.get_scale();
        let rotation = self.get_rotation();
        let translation = self.get_translation();


    let the_return = 
        base_transform
            * Matrix4::from_angle_z(rotation)
            * Matrix4::from_nonuniform_scale(scale[0], scale[1], 1.0)
            * Matrix4::from_translation(translation)
            * Matrix4::from_scale(1.0);

        the_return
    }
}

impl BoneTranslateKeyFrame
{
    pub fn get_translation(&self) -> Vector3<f32>
    {
        Vector3::new(self.x, self.y, 1.0)
    }
}

impl BoneScaleKeyFrame
{
    pub fn get_scale(&self) -> Vector2<f32>
    {
        Vector2::new(self.x, self.y)
    }
}
