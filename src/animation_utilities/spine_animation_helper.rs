use cgmath::Matrix4;
use super::super::storage_representation::Event;

use crate::storage_representation::{SpineModel, animations::Animation, bones::Bone, slots::Slot};

pub trait SpineAnimationHelper: Sync + Send
{
    fn get_bone_transform(&self, bone: &Bone, animation: &Animation, time: f32) -> Matrix4<f32>;
    fn get_slot_attachment(&self, slot: &Slot, animation: &Animation, time: f32) -> Option<String>;
    fn get_events(&self, model: &SpineModel, animation: &Animation, time: f32, duration: f32) -> Vec<Event>;
}
