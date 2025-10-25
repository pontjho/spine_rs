use crate::storage_representation::SpineModel;

pub trait SpineParser
{
    fn parse(&self, data: &str) -> Result<SpineModel, String>;
}

