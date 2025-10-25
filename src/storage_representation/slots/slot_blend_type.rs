use serde::Deserialize;

#[derive(Deserialize, Debug)]
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
