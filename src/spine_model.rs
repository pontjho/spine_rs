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
    pub slots: Vec<Slot>,
    pub skins: HashMap<String, Skin>,
    pub animations: HashMap<String, Animation>
}
