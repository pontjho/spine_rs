use super::animations::Animation;
use super::Skeleton;
use super::bones::Bone;
use super::slots::Slot;
use super::Skin;
use super::Event;
use std::collections::HashMap;

use serde::Deserialize;

#[derive(Deserialize, Debug)]
#[serde(rename_all(serialize = "camelCase", deserialize = "snake_case"))]
pub struct SpineModel
{
    pub skeleton: Skeleton,
    pub bones: Vec<Bone>,
    #[serde(default)]
    pub slots: Vec<Slot>,
    #[serde(default)]
    pub skins: Vec<Skin>,
    #[serde(default)]
    pub animations: HashMap<String, Animation>,
    #[serde(default)]
    pub events: HashMap<String, Event>
}
impl SpineModel {
    pub fn get_animations<'a>(&'a self, animation_ids: Vec<usize>) -> Vec<&'a Animation> {
        
        let animations: Vec<_> = animation_ids.into_iter().map(|animation_id| self.animations.iter().nth(animation_id).unwrap().1).collect();
        animations
    }
}
