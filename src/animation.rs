use std::collections::HashMap;
use crate::bone_keyframe::BoneKeyFrame;

use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Animation
{
    bones: HashMap<String, BoneKeyFrame>,
    slots: HashMap<String, BoneKeyFrame>,
    ik: HashMap<String, BoneKeyFrame>,
    deform: HashMap<String, BoneKeyFrame>,
    events: HashMap<String, BoneKeyFrame>,
    draworder: HashMap<String, BoneKeyFrame>,
}
