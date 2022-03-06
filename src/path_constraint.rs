use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all(serialize = "camelCase", deserialize = "snake_case"))]
pub struct PathConstraint
{
    pub name: String,
    pub order: usize,
    pub skin: bool,
    pub bones: Vec<String>,
    pub target: String,
    pub position_mode: PathConstraintPositionMode,
    pub spacing_mode: PathConstraintSpacingMode,
    pub rotate_mode: PathConstraintRotateMode,
    pub rotation: f32,
    pub position: f32,
    pub spacing: f32,
    pub rotate_mix: f32,
    pub translate_mix: f32
}

#[derive(Serialize, Deserialize, Debug)]
pub enum PathConstraintPositionMode
{
    Fixed,
    Percent
}

#[derive(Serialize, Deserialize, Debug)]
pub enum PathConstraintSpacingMode
{
    Length,
    Fixed,
    Percent
}

#[derive(Serialize, Deserialize, Debug)]
pub enum PathConstraintRotateMode
{
    Tangent,
    Chain,
    ChainScale
}
