use super::super::SpineParser;
use super::SpineModel;

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
