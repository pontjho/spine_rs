use crate::attachment::Attachment;
use crate::attachment::AttachmentType;
use std::collections::HashMap;

use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all(serialize = "camelCase", deserialize = "snake_case"))]
pub struct Skin
{
    #[serde(default="default")]
    pub name: String,
    #[serde(default)]
    pub attachments: HashMap<String, HashMap<String, AttachmentType>>
}

pub fn default() -> String
{
    "default".to_string()
}
