use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub enum PathConstraintSpacingMode
{
    Length,
    Fixed,
    Percent
}

impl Default for PathConstraintSpacingMode
{
    fn default() -> Self
    {
        PathConstraintSpacingMode::Length
    }
}
