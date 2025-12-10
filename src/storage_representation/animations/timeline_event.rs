use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all(serialize = "camelCase", deserialize = "snake_case"))]
pub struct TimelineEvent
{
    #[serde(default)]
    pub time: f32,
    pub name: String,
    
    #[serde(default, rename="int")]
    pub int_value: Option<isize>,

    #[serde(default, rename="float")]
    pub float_value: Option<f32>,

    #[serde(default, rename="string")]
    pub string_value: Option<String>,

    #[serde(default)]
    pub audio: Option<String>,

    #[serde(default)]
    pub volume: Option<f32>,

    #[serde(default)]
    pub balance: Option<f32>
}
