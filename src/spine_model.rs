use crate::animation::Animation;
use crate::skeleton::Skeleton;
use crate::bone::Bone;
use crate::slot::Slot;
use crate::skin::Skin;
use std::collections::HashMap;

pub struct SpineModel
{
    pub skeleton: Skeleton,
    pub bones: Vec<Bone>,
    pub slots: Vec<Slot>,
    pub skins: HashMap<String, Skin>,
    pub animations: HashMap<String, Animation>
}
