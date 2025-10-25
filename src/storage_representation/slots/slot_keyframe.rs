use serde::Deserialize;
use super::SlotAttachmentKeyFrame;
use super::SlotColourKeyFrame;

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all(serialize = "camelCase", deserialize = "snake_case"))]
pub struct SlotKeyFrame
{
    #[serde(default)]
    pub attachment: Vec<SlotAttachmentKeyFrame>,
    #[serde(default)]
    pub colour: Vec<SlotColourKeyFrame>
}
