use crate::attachment::Attachment;
use std::collections::HashMap;

pub struct Skin
{
    pub name: String,
    pub attachments: HashMap<String, Attachment>
}
