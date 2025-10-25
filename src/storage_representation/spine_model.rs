use super::animations::Animation;
use super::skeletons::Skeleton;
use super::bones::Bone;
use super::slots::Slot;
use super::skins::Skin;
use std::collections::HashMap;

use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
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
    pub animations: HashMap<String, Animation>
}
