use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct TransformConstraint
{
    pub name: String,
    pub order: usize,
    pub skin: bool,
    pub bones: Vec<String>,
    pub target: String,
    pub rotation: f32,
    pub x: f32,
    pub y: f32,
    pub scale_x: f32,
    pub scale_y: f32,
    pub shear_y: f32,
    pub rotate_mix: f32,
    pub translate_mix: f32,
    pub scale_mix: f32,
    pub shear_mix: f32,
    pub local: bool,
    pub relative: bool
}
