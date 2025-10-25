use serde::{Serialize, Deserialize};
use crate::util::ffffffff;
use crate::util::deserialize_colour;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all(serialize = "camelCase", deserialize = "snake_case"))]
pub struct Slot
{
    pub name: String,
    pub bone: String,

    #[serde(default="ffffffff", deserialize_with="deserialize_colour")]
    pub color: u32,
    pub dark: Option<u32>,
    pub attachment: Option<String>,

    #[serde(default)]
    pub blend: SlotBlendType
}

#[derive(Serialize, Deserialize, Debug)]
pub enum SlotBlendType
{
    Normal,
    Additive,
    Multiply,
    Screen
}

impl Default for SlotBlendType
{
    fn default() -> Self
    {
        SlotBlendType::Normal
    }
}
