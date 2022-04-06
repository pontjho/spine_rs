use crate::bone::Bone;
use cgmath::prelude::*;
use cgmath::Vector2;
use cgmath::Matrix3;
use cgmath::Rad;
use cgmath::Deg;
use crate::bone_keyframe::BoneTranslateKeyFrame;
use crate::bone_keyframe::BoneScaleKeyFrame;
use crate::bone_keyframe::BoneShearKeyFrame;
use crate::bone_keyframe::BoneRotateKeyFrame;
use crate::attachment::RegionAttachment;

pub trait CGMathIntegrations
{
    fn get_translation(&self) -> Vector2<f32>;
    fn get_rotation(&self) -> Deg<f32>;
    fn get_scale(&self) -> Vector2<f32>;
    fn get_shear(&self) -> Vector2<f32>;

    //fn get_transform(&self) -> Matrix3<f32>;
}

// pub trait AnimationLogic
// {
//     fn get_translation_at(&self, time: f32, keyframes: &Vec<BoneTranslateKeyFrame>) -> Vector2<f32>;
//     fn get_scale_at(&self, time: f32, keyframes: &Vec<BoneScaleKeyFrame>) -> Vector2<f32>;
//     fn get_shear_at(&self, time: f32, keyframes: &Vec<BoneShearKeyFrame>) -> Vector2<f32>;
//     fn get_rotation_at(&self, time: f32, keyframes: &Vec<BoneRotateKeyFrame>) -> Rad<f32>;
// }

// pub trait Matrixer
// {
//     fn build_transform(&self, translation: (f32, f32), scale: (f32, f32), shear: (f32, f32), rotation: f32) -> Matrix3<f32>;
// }

pub trait InterpolationLogic
{
    fn interpolate_translation(&self, time: f32, from: BoneTranslateKeyFrame, to: BoneTranslateKeyFrame) -> (f32, f32);
    fn interpolate_scale(&self, time: f32, from: BoneScaleKeyFrame, to: BoneScaleKeyFrame) -> (f32, f32);
    fn interpolate_shear(&self, time: f32, from: BoneShearKeyFrame, to: BoneShearKeyFrame) -> (f32, f32);
    fn interpolate_rotation(&self, time: f32, from: BoneRotateKeyFrame, to: BoneRotateKeyFrame) -> f32;
}

impl CGMathIntegrations for Bone
{
    fn get_translation(&self) -> Vector2<f32>
    {
        // println!("Translation for {} is {}, {}", self.name, self.x, self.y);
        Vector2::new(self.x, self.y)
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
    fn get_translation(&self) -> Vector2<f32>
    {
        Vector2::new(self.x, self.y)
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
    pub fn get_transform(&self, attachment_name: &str) -> Matrix3<f32>
    {
        let scale = self.get_scale();
        let rotation = self.get_rotation();
        let translation = self.get_translation();

        // let scale_matrix = Matrix3::from_nonuniform_scale(self.scale_x, self.scale_y);
        // let rotation_matrix = Matrix3::from_angle_z(rotation);
        // let translation_matrix = Matrix3::from_translation(translation);
        
        // let the_return = scale_matrix
        //     * rotation_matrix
        //     * translation_matrix;

        // println!("Build transform for {} with {:?}, {:?}, {:?} is {:?}", attachment_name, scale, rotation, translation, the_return);
        let the_return = create_transform(rotation, translation, scale);

        the_return
    }
}

impl BoneTranslateKeyFrame
{
    pub fn get_translation(&self) -> Vector2<f32>
    {
        Vector2::new(self.x, self.y)
    }
}

impl BoneScaleKeyFrame
{
    pub fn get_scale(&self) -> Vector2<f32>
    {
        Vector2::new(self.x, self.y)
    }
}

pub fn create_transform(rotation: Deg<f32>, translation: Vector2<f32>, scale: Vector2<f32>) -> Matrix3<f32>
{
    let the_return = 
            Matrix3::from_scale(1.0)
            * Matrix3::from_translation(translation)
            * Matrix3::from_angle_z(rotation)
            * Matrix3::from_nonuniform_scale(scale[0], scale[1])
            * Matrix3::from_scale(1.0);
    //println!("Creating bone {} transform from translation {:?} and rotation {:?} as {:?}", bone.name, translation, rotation, the_return);
    the_return
}

// pub fn get_bone_transform(bone: &Bone, bone_animations: &BoneKeyFrame, time: f32)
// {
//     // let animation = animations[animation];
//     // let rotation_index = animation.translations.iter().find(|a| a.time < time);
//     // let shear_index = animation.translations.iter().find(|a| a.time < time);
//     // let scale_index = animation.translations.iter().find(|a| a.time < time);

//     let translation = find_animation_translation(bone_animations)
//         .map(|v| {
//             let (trans1, trans2) = find_nearest_translations(animation.translations.iter().find_index(|a| a.time < time));
//             let translate = interpolate_translation(trans1, trans2, time);
//         })
//         .unwrap_or(bone.get_translation());
// }