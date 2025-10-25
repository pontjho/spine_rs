use super::animations::Animation;
use super::Skeleton;
use super::bones::Bone;
use super::slots::Slot;
use super::Skin;
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
    pub animations: HashMap<String, Animation>
}
