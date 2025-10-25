use serde::{self, Deserialize, Deserializer};

pub fn one() -> f32
{
    1.0
}

pub fn ffffffff() -> u32
{
    0xFFFFFFFF
}

pub fn troo() -> bool
{
    true
}

pub fn deserialize_colour<'de, D>(deserializer: D) -> Result<u32, D::Error> where D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer).unwrap();
    let val = u32::from_str_radix(&s, 16).unwrap();
    let val_adjusted = if s.len() == 6 { (val << 2) | 0xFF } else { val };
    Ok(val_adjusted)
}
