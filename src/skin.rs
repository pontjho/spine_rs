use crate::attachment::Attachment;
use std::collections::HashMap;

use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Skin
{
    pub name: String,
    pub attachments: HashMap<String, Attachment>
}
