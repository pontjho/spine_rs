use std::fs;
use spine_rs::spine_parser::{ConcreteSpineParser, SpineParser};
use spine_rs::spine_manager::{SpineManager, ConcreteSpineManager, ConcreteSpineAnimationHelper};

fn main() {
    let v = fs::read_to_string("example/test_model.json").unwrap();
    let spine_parser = ConcreteSpineParser {};
    let model = spine_parser.parse(&v).unwrap();

    let spine_manager = ConcreteSpineManager {
        animator: Box::from(ConcreteSpineAnimationHelper {})
    };
    //time: f32, model: &SpineModel, animation_name: &str, with_skin: &str
    let attachments = spine_manager.get_attachments_at(0.0, &model, "translate_test", "default");

    println!("{:?}", attachments);
}
