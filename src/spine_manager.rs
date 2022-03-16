use crate::spine_model::SpineModel;
use crate::attachment::Attachment;
use crate::bone::Bone;
use crate::animation::Animation;
use crate::attachment::AttachmentType::Region;
use crate::bone_keyframe::BoneKeyFrame;
use crate::cgmath_integration::CGMathIntegrations;
use cgmath::prelude::*;
use cgmath::Vector2;
use cgmath::Matrix3;
use std::collections::HashMap;

pub trait SpineManager
{
    fn get_attachments_at(&self, time: f32, from_model: &SpineModel, with_animation: &str, with_skin: &str) -> Vec<ModelImage>;
}

pub trait SpineAnimationHelper
{
    fn get_bone_transform(&self, bone: &Bone, animation: &Animation, time: f32) -> Matrix3<f32>;
}

pub struct ConcreteSpineManager
{
    pub animator: Box<dyn SpineAnimationHelper>
}

pub struct ConcreteSpineAnimationHelper
{

}

impl SpineManager for ConcreteSpineManager
{
    fn get_attachments_at(&self, time: f32, model: &SpineModel, animation_name: &str, with_skin: &str) -> Vec<ModelImage>
    {
        let animation = &model.animations[animation_name];
        let bone_global_transforms: HashMap<String, Matrix3<f32>> = model
            .bones
            .iter()
            .map(|bone| (bone.name.clone(), bone.parent.clone(), self.animator.get_bone_transform(bone, animation, time)))
        //     .collect();
        // let bone_global_transforms: HashMap<String, Matrix3<f32>> = bone_local_transforms
        //     .iter()
            .fold(HashMap::default(), |accum, (bone_name, parent_bone, transform)| {
                let bone_transform = parent_bone.as_ref().map(|b| accum[b] * transform).unwrap_or(transform.clone());
                let new_transform = vec![(bone_name.to_string(), bone_transform)];
                accum.into_iter().chain(new_transform).collect()
            });

        let skin = model.skins.iter().find(|s| s.name == with_skin).unwrap();
        let skin_attachments = &skin.attachments;
        let active_attachments: Vec<_> = model
            .slots
            .iter()
            .map(|v| (v.name.clone(), v.bone.clone(), v.attachment.clone()))
            // .collect();
        // let active_attachments: Vec<_> = active_attachment_names
            // .iter()
            .filter_map(|(slot_name, bone_name, attachment_name)| attachment_name.map(|aname| (slot_name, bone_name, aname)))
            .map(|(slot_name, bone_name, attachment_name)| (bone_name, attachment_name.clone(), skin_attachments[&slot_name][&attachment_name].clone()))
            .collect();

        let images: Vec<_> = active_attachments
            .into_iter()
            .filter_map(|(bone_name, attachment_name, attachment)| match attachment {
                Region(a) => {
                    let aa: Matrix3<f32> = a.get_transform();
                    let b: Matrix3<f32> = bone_global_transforms[&bone_name];
                    Some(ModelImage { transform: (aa * b).into(), texture_name: a.path.unwrap_or(attachment_name)})
                },
                _ => None
            })
            .collect();

        images
    }
}

#[derive(Debug, Clone)]
pub struct ModelImage
{
    pub transform: [[f32; 3]; 3],
    pub texture_name: String
}

impl SpineAnimationHelper for ConcreteSpineAnimationHelper
{
    fn get_bone_transform(&self, bone: &Bone, animation: &Animation, time: f32) -> Matrix3<f32>
    {
        animation.bones.get(&bone.name)
            .map(|BoneKeyFrame { rotate: rotations, translate: translations, scale: scales, shear: shears }| {
                let rotation = if rotations.len() > 0
                {
                    let rotation_far_index = rotations.iter().enumerate().filter(|(_, v)| v.time <= time).count();
                    let rotation_near_index = if rotation_far_index == 0 { 0 } else { rotation_far_index - 1 };
                    let ref near_rotation = rotations[rotation_near_index];
                    let ref far_rotation = rotations[rotation_far_index];
                    let near_angle = near_rotation.angle;
                    let far_angle = far_rotation.angle;
                    let interval_length = far_rotation.time - near_rotation.time;
                    let normalised_time = time - near_rotation.time;
                    let near_rotation_weight = normalised_time / interval_length;
                    let far_rotation_weight = 1.0 - near_rotation_weight;
                    let rotation_deg = near_angle * near_rotation_weight + far_angle * far_rotation_weight;
                    cgmath::Deg(rotation_deg).into()
                }
                else
                {
                    bone.get_rotation()
                };

                let translation = if translations.len() > 0
                {
                    let translation_far_index = translations.iter().filter(|v| v.time <= time).count();
                    let translation_far_index = if translation_far_index == translations.len() { translation_far_index - 1 } else { translation_far_index };
                    let translation_near_index = if translation_far_index == 0 { 0 } else { translation_far_index - 1 };
                    let ref near_translation = translations[translation_near_index];
                    let ref far_translation = translations[translation_far_index];
                    let near_position = near_translation.get_translation();
                    let far_position = far_translation.get_translation();

                    let translation = if near_position == far_position
                    {
                        println!("time: {}\r\n {:#?}", time, translations);
                        near_position
                    }
                    else
                    {
                        let interval_length = far_translation.time - near_translation.time;
                        let normalised_time = time - near_translation.time;
                        let far_translation_weight = normalised_time / interval_length;
                        let near_translation_weight = 1.0 - far_translation_weight;
                        let t = near_position * near_translation_weight + far_position * far_translation_weight;
                        println!("******\r\n near: {} \r\n far: {} \r\n interval: {} \r\n time: {} \r\n near_weight: {} \r\n far_weight: {} \r\n result {:#?} \r\n inputs: {:#?}\r\n*********", translation_near_index, translation_far_index, interval_length, normalised_time, near_translation_weight, far_translation_weight, t, translations);
                        t
                    };

                    translation
                }
                else
                {
                    bone.get_translation()
                };

                Matrix3::from_angle_z(rotation)
                    * Matrix3::from_translation(translation)
            })
            .unwrap_or(Matrix3::identity())
    }
}
