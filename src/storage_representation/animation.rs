use std::collections::HashMap;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all(serialize = "camelCase", deserialize = "snake_case"))]
pub struct Animation
{
    #[serde(default)]
    pub bones: HashMap<String, BoneKeyFrame>,

    #[serde(default)]
    pub slots: HashMap<String, SlotKeyFrame>,
    //ik: HashMap<String, BoneKeyFrame>,
    //deform: HashMap<String, BoneKeyFrame>,
    //events: HashMap<String, BoneKeyFrame>,
    //draw_order: HashMap<String, BoneKeyFrame>,
}
