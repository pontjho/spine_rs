use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Slot
{
    pub name: String,
    pub bone: String,
    pub color: f32,
    pub dark: String,
    pub attachment: String,
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
