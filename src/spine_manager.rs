use crate::spine_model::SpineModel;
use crate::bone::Bone;
use crate::slot::Slot;
use crate::animation::Animation;
use crate::attachment::AttachmentType::Region;
use crate::bone_keyframe::BoneKeyFrame;
use crate::slot_keyframe::SlotKeyFrame;
use crate::cgmath_integration::CGMathIntegrations;
use cgmath::Vector2;
use cgmath::Matrix4;
use cgmath::Vector3;
use std::collections::HashMap;
use crate::bone_keyframe::BoneTranslateKeyFrame;
use crate::bone_keyframe::BoneRotateKeyFrame;
use cgmath::Deg;
use std::cmp::Ordering;
use crate::bone_keyframe::BoneScaleKeyFrame;
use crate::cgmath_integration::create_transform;
use crate::attachment::AttachmentType;

pub trait SpineManager
{
    fn get_attachments_at(&self, time: f32, from_model: &SpineModel, with_animation: &str, with_skin: &str) -> Vec<ModelImage>;
    fn get_animation_id_attachments_at(&self, time: f32, from_model: &SpineModel, with_animation: usize, with_skin: &str) -> Vec<ModelImage>;

    fn mix_animations(&self, animations: &Vec<Animation>) -> Animation;
    fn get_attachments_for_animation(&self, time: f32, from_model: &SpineModel, with_animation: &Animation, with_skin: &str) -> Vec<ModelImage>;
    fn get_bounding_boxes_for_animation(&self, time: f32, from_model: &SpineModel, with_animation: &Animation, with_skin: &str) -> Vec<BoundingBox>;
}

pub trait SpineAnimationHelper
{
    fn get_bone_transform(&self, bone: &Bone, animation: &Animation, time: f32) -> Matrix4<f32>;
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
    fn get_animation_id_attachments_at(&self, time: f32, model: &SpineModel, animation_id: usize, with_skin: &str) -> Vec<ModelImage>
    {
        let animation_name = model.animations.iter().nth(animation_id).unwrap().0;
        let the_return = self.get_attachments_at(time, model, animation_name, with_skin);
        the_return
    }

    fn get_attachments_at(&self, time: f32, model: &SpineModel, animation_name: &str, with_skin: &str) -> Vec<ModelImage>
    {
        let animation = &model.animations[animation_name];
        self.get_attachments_for_animation(time, model, animation, with_skin)
    }

    fn mix_animations(&self, animations: &Vec<Animation>) -> Animation
    {
        let mut bones: HashMap<String, BoneKeyFrame> = Default::default();
        let mut slots: HashMap<String, SlotKeyFrame> = Default::default();

        for Animation { bones: a_bones, slots: a_slots } in animations
        {
            for (bone_name, bone) in a_bones
            {
                if !bones.contains_key(bone_name)
                {
                    bones.insert(bone_name.to_string(), bone.clone());
                }
            }
            for (slot_name, slot) in a_slots
            {
                if !slots.contains_key(slot_name)
                {
                    slots.insert(slot_name.to_string(), slot.clone());
                }
            }
        }

        Animation {
            bones,
            slots
        }
    }

    fn get_attachments_for_animation(&self, time: f32, model: &SpineModel, animation: &Animation, with_skin: &str) -> Vec<ModelImage>
    {
        let bone_global_transforms = self.get_bone_transforms(time, model, animation);
        let active_attachments: Vec<_> = self.get_active_attachments(time, model, animation, with_skin);
        let attachment_transforms = self.get_attachment_transforms(active_attachments, bone_global_transforms);
        let images: Vec<_> = self.get_attachment_images(attachment_transforms);
        images
    }
}



#[derive(Debug, Clone)]
pub struct ModelImage
{
    pub transform: [[f32; 4]; 4],
    pub dimensions: (f32, f32),
    pub texture_name: String,
    pub vertices: Vec<SpineVertex>, // This is to allow for freeform deformation
    pub indices: Vec<u32>
}

#[derive(Debug, Clone)]
pub struct SpineVertex
{
    pub position: [f32; 3],
    pub uv: [f32; 2]
}

pub fn dimensions_as_vertices(dimensions: (f32, f32), padding: [f32; 4]) -> [([f32; 3], [f32; 2]); 6]
{
    let (w, h) = dimensions;
    let (half_width, half_height) = (w / 2.0, h / 2.0);

    let [left_padding, top_padding, right_padding, bottom_padding] = padding;

    let denominator = 1.0;//2500.0;
    let (left, right) = (-half_width + left_padding, half_width - right_padding);
    let (top, bottom) = (half_height - top_padding, -half_height + bottom_padding);

    let left = left / denominator;
    let right = right / denominator;
    let top = top / denominator;
    let bottom = bottom / denominator;

    let u_left = 0.0;
    let u_right = 1.0;
    let u_top = 0.0;
    let u_bottom = 1.0;
    
    let vertices = [

        ([left, top, 0.0], [u_left, u_top]),
        ([left, bottom, 0.0], [u_left, u_bottom]),
        ([right, bottom, 0.0], [u_right, u_bottom]),
        ([right, bottom, 0.0], [u_right, u_bottom]),
        ([right, top, 0.0], [u_right, u_top]),
        ([left, top, 0.0], [u_left, u_top]),
    ];

    // let v1 = [
    //     [-0.5, 0.5, 0.0],
    //     [-0.5, -0.5, 0.0],
    //     [0.5, -0.5, 0.0],
    //     [0.5, -0.5, 0.0],
    //     [0.5, 0.5, 0.0],
    //     [-0.5, 0.5, 0.0],];
    vertices
}

pub fn dimensions_as_vertices_bottom_left_aligned(dimensions: (f32, f32), _padding: [f32; 4]) -> [[f32; 3]; 4]
{
    let (w, h) = dimensions;
    let (half_width, _half_height) = (w / 2.0, h / 2.0);
    let (left, right) = (-half_width, half_width);
    let (top, bottom) = (h, 0.0);
    let vertices = [
        [left, top, 1.0],
        [right, top, 1.0],
        [right, bottom, 1.0],
        [left, bottom, 1.0]
    ];
    vertices
}

pub trait Interpolatable
{
    type Value: std::ops::Mul<f32, Output = Self::Value> + std::ops::Add<Self::Value, Output = Self::Value> + core::fmt::Debug;

    fn time(&self) -> f32;
    fn get_value(&self) -> Self::Value;
}

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

impl Interpolatable for BoneScaleKeyFrame
{
    type Value = Vector2<f32>;

    fn time(&self) -> f32
    {
        self.time
    }

    fn get_value(&self) -> Self::Value
    {
        self.get_scale()
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
        near_position
    }
    else
    {
        let interval_length = far_translation.time() - near_translation.time();
        let normalised_time = time - near_translation.time();
        let far_translation_weight = normalised_time / interval_length;
        let near_translation_weight = 1.0 - far_translation_weight;
        let t = near_position * near_translation_weight + far_position * far_translation_weight;
        t
    };

    translation
}

impl SpineAnimationHelper for ConcreteSpineAnimationHelper
{
    fn get_bone_transform(&self, bone: &Bone, animation: &Animation, time: f32) -> Matrix4<f32>
    {
        let (rotation, translation, scale) = animation.bones.get(&bone.name)
            .map(|BoneKeyFrame { rotate: rotations, translate: translations, scale: scales, shear: _shears }| {
                let rotation = if rotations.len() > 0
                {
                    interpolate(time, rotations) + bone.get_rotation()
                }
                else
                {
                    bone.get_rotation()
                };

                let translation = if translations.len() > 0
                {
                    interpolate(time, translations) + bone.get_translation()
                }
                else
                {
                    bone.get_translation()
                };

                let scale = if scales.len() > 0
                {
                    let interp_scale = interpolate(time, scales);
                    let bone_scale = bone.get_scale();
                    Vector2::new(interp_scale.x * bone_scale.x, interp_scale.y * bone_scale.y)
                }
                else
                {
                    bone.get_scale()
                };

                (rotation, translation, scale)
            })
            .unwrap_or((bone.get_rotation(), bone.get_translation(), bone.get_scale()));
            //.unwrap_or(Matrix3::from_angle_z(bone.get_rotation()) * Matrix3::from_nonuniform_scale(bone.scale_x, bone.scale_y) * Matrix3::from_translation(bone.get_translation()));

        if bone.name == "root"
        {
            //println!("+++++++++++++{} {:?} {:?} {:?}", time, rotation, translation, scale)
        }

        let the_return = create_transform(rotation, translation, scale);

        the_return
    }

    fn get_slot_attachment(&self, slot: &Slot, animation: &Animation, time: f32) -> Option<String>
    {
        animation.slots.get(&slot.name)
            .map(|SlotKeyFrame { attachment: animation_slots, colour: _ }| {
                // println!("Anim {} {:?} --- {:?} ---", time, animation_slots, slot.attachment);

                let the_return = animation_slots
                    .iter()
                    .filter(|v| v.time <= time) //This is super inefficient
                    .last()
                    .map(|v| &v.attachment_name)
                    .unwrap_or(&slot.attachment)
                    .clone();

                // println!("And the winner is {:?}", the_return);
                the_return
            })
            .unwrap_or(slot.attachment.clone())
    }
}
