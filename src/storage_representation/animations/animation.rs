use std::collections::HashMap;
use serde::Deserialize;

use crate::storage_representation::animations::TimelineEvent;

use super::super::bones::BoneKeyFrame;
use super::super::slots::SlotKeyFrame;

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all(serialize = "camelCase", deserialize = "snake_case"))]
pub struct Animation
{
    #[serde(default)]
    pub bones: HashMap<String, BoneKeyFrame>,

    #[serde(default)]
    pub slots: HashMap<String, SlotKeyFrame>,

    #[serde(default)]
    pub events: Vec<TimelineEvent>
    //ik: HashMap<String, BoneKeyFrame>,
    //deform: HashMap<String, BoneKeyFrame>,
    //events: HashMap<String, BoneKeyFrame>,
    //draw_order: HashMap<String, BoneKeyFrame>,
}
