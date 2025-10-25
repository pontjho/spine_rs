//use serde::{Serialize, Deserialize};
use serde::{self, Serialize, Deserialize, Deserializer};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all(serialize = "camelCase", deserialize = "snake_case"))]
pub enum AnimationInterpolation
{
    Linear,
    Stepped,
    ControlPoints(Vec<f32>)
}

impl Default for AnimationInterpolation
{
    fn default() -> Self
    {
        AnimationInterpolation::Linear
    }
}

pub fn deserialize_animation_interpolation<'de, D>(deserializer: D) -> Result<AnimationInterpolation, D::Error> where D: Deserializer<'de>,
{
    let _s = String::deserialize(deserializer)?;
    //Utc.datetime_from_str(&s, FORMAT).map_err(serde::de::Error::custom)
    Ok(AnimationInterpolation::Linear)
}
