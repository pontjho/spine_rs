pub struct Slot
{
    pub name: String,
    pub bone: String,
    pub color: f32,
    pub dark: String,
    pub attachment: String,
    pub blend: SlotBlendType
}

pub enum SlotBlendType
{
    Normal,
    Additive,
    Multiply,
    Screen
}
