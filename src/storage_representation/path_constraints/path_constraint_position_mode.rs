use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub enum PathConstraintPositionMode
{
    Fixed,
    Percent
}

impl Default for PathConstraintPositionMode
{
    fn default() -> Self
    {
        PathConstraintPositionMode::Percent
    }
}
