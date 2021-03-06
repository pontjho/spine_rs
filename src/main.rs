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

    let timesteps = [0.0, 0.5, 1.0];
    for timestep in timesteps
    {
        let animation_name = "translate_test";
        let attachments = spine_manager.get_attachments_at(timestep * 0.3333, &model, animation_name, "default");
        println!("{} \t\t {:?}", animation_name, attachments);

        let animation_name = "rotate_test";
        let attachments = spine_manager.get_attachments_at(timestep * 0.3333, &model, animation_name, "default");
        println!("{} \t\t {:?}", animation_name, attachments);

        let animation_name = "slot_change_test";
        let attachments = spine_manager.get_attachments_at(timestep * 0.3333, &model, animation_name, "default");
        println!("{} \t\t {:?}", animation_name, attachments);

        println!("\r\n");
    }
}
