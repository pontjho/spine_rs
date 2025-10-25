use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub enum PathConstraintRotateMode
{
    Tangent,
    Chain,
    ChainScale
}

impl Default for PathConstraintRotateMode
{
    fn default() -> Self
    {
        PathConstraintRotateMode::Tangent
    }
}
