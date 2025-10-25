use cgmath::Matrix4;

use crate::storage_representation::{animations::Animation, bones::Bone, slots::Slot};

pub trait SpineAnimationHelper
{
    fn get_bone_transform(&self, bone: &Bone, animation: &Animation, time: f32) -> Matrix4<f32>;
    fn get_slot_attachment(&self, slot: &Slot, animation: &Animation, time: f32) -> Option<String>;
}
