use cgmath::{Matrix4, Vector2};

use crate::{animation_utilities::{interpolation::interpolate, spine_animation_helper::SpineAnimationHelper}, cgmath_integration::CGMathIntegrations, storage_representation::{animations::Animation, bones::{Bone, BoneKeyFrame}, slots::{Slot, SlotKeyFrame}}};
use crate::animation_utilities::create_transform::create_transform;

pub struct ConcreteSpineAnimationHelper
{

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
