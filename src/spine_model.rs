use crate::animation::Animation;
use crate::skeleton::Skeleton;
use crate::bone::Bone;
use crate::slot::Slot;
use crate::skin::Skin;
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
