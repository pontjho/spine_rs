use crate::spine_model::SpineModel;

pub trait SpineParser
{
    fn parse(&self, data: &str) -> Result<SpineModel, String>;
}

pub struct ConcreteSpineParser
{

}

impl SpineParser for ConcreteSpineParser
{
    fn parse(&self, data: &str) -> Result<SpineModel, String>
    {
        let p: SpineModel = serde_json::from_str(data).unwrap();

        Ok(p)
    }
}
