use crate::spine_model::SpineModel;
use crate::attachment::Attachment;
use crate::bone::Bone;
use crate::slot::Slot;
use crate::animation::Animation;
use crate::attachment::AttachmentType::Region;
use crate::bone_keyframe::BoneKeyFrame;
use crate::slot_keyframe::SlotKeyFrame;
use crate::cgmath_integration::CGMathIntegrations;
use cgmath::prelude::*;
use cgmath::Vector2;
use cgmath::Matrix3;
use std::collections::HashMap;
use crate::bone_keyframe::BoneTranslateKeyFrame;
use crate::bone_keyframe::BoneRotateKeyFrame;
use cgmath::Rad;

pub trait SpineManager
{
    fn get_attachments_at(&self, time: f32, from_model: &SpineModel, with_animation: &str, with_skin: &str) -> Vec<ModelImage>;
    fn get_animation_id_attachments_at(&self, time: f32, from_model: &SpineModel, with_animation: usize, with_skin: &str) -> Vec<ModelImage>;
}

pub trait SpineAnimationHelper
{
    fn get_bone_transform(&self, bone: &Bone, animation: &Animation, time: f32) -> Matrix3<f32>;
    fn get_slot_attachment(&self, slot: &Slot, animation: &Animation, time: f32) -> Option<String>;
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
            // .map(|v| (v.name.clone(), v.bone.clone(), v.attachment.clone()))
            .map(|v| (v.name.clone(), v.bone.clone(), self.animator.get_slot_attachment(v, animation, time)))
            .filter_map(|(slot_name, bone_name, attachment_name)| attachment_name.map(|aname| (slot_name, bone_name, aname)))
            .map(|(slot_name, bone_name, attachment_name)| (bone_name, attachment_name.clone(), skin_attachments[&slot_name][&attachment_name].clone()))
            .collect();

        // println!("*********{:?}------------", active_attachments);

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

        // println!("{:#?}", images);
        images
    }

    fn get_animation_id_attachments_at(&self, time: f32, model: &SpineModel, animation_id: usize, with_skin: &str) -> Vec<ModelImage>
    {
        let animation_name = model.animations.iter().nth(animation_id).unwrap().0;
        self.get_attachments_at(time, model, animation_name, with_skin)
    }
}

#[derive(Debug, Clone)]
pub struct ModelImage
{
    pub transform: [[f32; 3]; 3],
    pub texture_name: String
}

pub trait Interpolatable
{
    type Value: std::ops::Mul<f32, Output = Self::Value> + std::ops::Add<Self::Value, Output = Self::Value> + core::fmt::Debug;

    fn time(&self) -> f32;
    fn get_value(&self) -> Self::Value;
}

impl Interpolatable for BoneTranslateKeyFrame
{
    type Value = Vector2<f32>;

    fn time(&self) -> f32
    {
        self.time
    }

    fn get_value(&self) -> Self::Value
    {
        self.get_translation()
    }
}

impl Interpolatable for BoneRotateKeyFrame
{
    type Value = Rad<f32>;

    fn time(&self) -> f32
    {
        self.time
    }

    fn get_value(&self) -> Self::Value
    {
        cgmath::Deg(self.angle).into()
    }
}

fn interpolate<T>(time: f32, items: &Vec<T>) -> T::Value where T: Interpolatable, T: core::fmt::Debug//, R: std::ops::Mul<f32>
{
    let translation_far_index = items.iter().filter(|v| v.time() <= time).count();
    let translation_far_index = if translation_far_index == items.len() { translation_far_index - 1 } else { translation_far_index };
    let translation_near_index = if translation_far_index == 0 { 0 } else { translation_far_index - 1 };
    let ref near_translation = items[translation_near_index];
    let ref far_translation = items[translation_far_index];
    let near_position = near_translation.get_value();
    let far_position = far_translation.get_value();

    let translation = if translation_far_index == translation_near_index
    {
        // println!("time: {}\r\n {:#?}", time, items);
        near_position
    }
    else
    {
        let interval_length = far_translation.time() - near_translation.time();
        let normalised_time = time - near_translation.time();
        let far_translation_weight = normalised_time / interval_length;
        let near_translation_weight = 1.0 - far_translation_weight;
        let t = near_position * near_translation_weight + far_position * far_translation_weight;
        // println!("******\r\n near: {} \r\n far: {} \r\n interval: {} \r\n time: {} \r\n near_weight: {} \r\n far_weight: {} \r\n result {:#?} \r\n inputs: {:#?}\r\n*********", translation_near_index, translation_far_index, interval_length, normalised_time, near_translation_weight, far_translation_weight, t, items);
        t
    };

    translation
}

impl SpineAnimationHelper for ConcreteSpineAnimationHelper
{
    fn get_bone_transform(&self, bone: &Bone, animation: &Animation, time: f32) -> Matrix3<f32>
    {
        animation.bones.get(&bone.name)
            .map(|BoneKeyFrame { rotate: rotations, translate: translations, scale: scales, shear: shears }| {
                let rotation = if rotations.len() > 0
                {
                    interpolate(time, rotations)
                }
                else
                {
                    bone.get_rotation()
                };

                let translation = if translations.len() > 0
                {
                    interpolate(time, translations)
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

    fn get_slot_attachment(&self, slot: &Slot, animation: &Animation, time: f32) -> Option<String>
    {
        // let animation_slots = animation.slots.get(&slot.name);
        // if animation_slots.len
        //     {
        //         let slot_keyframe_index = animation_slots.iter().filter(|v| v.time() <= time).count();
        //         let ref slot = animation_slots[slot_keyframe_index];
        //         slot.attachment
        //     })
        //     .unwrap_or(slot.attachment.clone())
        animation.slots.get(&slot.name)
            .map(|SlotKeyFrame { attachment: animation_slots, colour: _ }| {
                if animation_slots.len() > 0
                {
                    let slot_keyframe_index = animation_slots.iter().filter(|v| v.time <= time).count() - 1;
                    // println!("{} {} {} {} {}", time, animation_slots[0].time, animation_slots[0].time <= time, animation_slots[1].time <= time,slot_keyframe_index);
                    //let slot_keyframe_index = if slot_keyframe_index == animation_slots.len() { slot_keyframe_index - 1 } else { slot_keyframe_index };
                    let ref slotto = animation_slots[slot_keyframe_index];
                    let r = slotto.attachment_name.clone();
                    // println!("time: {}\r\n index: {} \r\n attachment: {:?} \r\n slot: {:?} \r\n slots: {:?}", time, slot_keyframe_index, r, slotto, animation_slots);
                    r
                }
                else
                {
                    slot.attachment.clone()
                }
            })
            .unwrap_or(slot.attachment.clone())
    }
}
