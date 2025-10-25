use serde::Deserialize;

use super::PathConstraintPositionMode;
use super::PathConstraintRotateMode;
use super::PathConstraintSpacingMode;

#[derive(Deserialize, Debug)]
#[serde(rename_all(serialize = "camelCase", deserialize = "snake_case"))]
pub struct PathConstraint
{
    pub name: String,
    pub order: usize,

    #[serde(default)]
    pub skin: bool,
    pub bones: Vec<String>,
    pub target: String,
    pub position_mode: PathConstraintPositionMode,
    pub spacing_mode: PathConstraintSpacingMode,
    pub rotate_mode: PathConstraintRotateMode,

    #[serde(default)]
    pub rotation: f32,

    #[serde(default)]
    pub position: f32,

    #[serde(default)]
    pub spacing: f32,

    #[serde(default)]
    pub rotate_mix: f32,

    #[serde(default)]
    pub translate_mix: f32
}
