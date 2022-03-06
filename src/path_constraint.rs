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

pub enum PathConstraintPositionMode
{
    Fixed,
    Percent
}

pub enum PathConstraintSpacingMode
{
    Length,
    Fixed,
    Percent
}

pub enum PathConstraintRotateMode
{
    Tangent,
    Chain,
    ChainScale
}
