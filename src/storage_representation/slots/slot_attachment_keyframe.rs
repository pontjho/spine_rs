use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all(serialize = "camelCase", deserialize = "snake_case"))]
pub struct SlotAttachmentKeyFrame
{
    #[serde(default)]
    pub time: f32,

    #[serde(alias = "name")]
    pub attachment_name: Option<String>
}
