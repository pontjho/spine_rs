use serde::Deserialize;
use super::AttachmentType;

#[derive(Deserialize, Debug)]
#[serde(rename_all(serialize = "camelCase", deserialize = "snake_case"))]
pub struct Attachment
{
    pub name: String,
    #[serde(alias = "type")]
    pub attachment_type: AttachmentType
}



// impl Default for AttachmentType
// {
//     fn default() -> Self
//     {
//         AttachmentType::Region
//     }
// }









