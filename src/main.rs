use std::fs;
use spine_rs::spine_parser::{ConcreteSpineParser, SpineParser};
use spine_rs::spine_manager::{SpineManager, ConcreteSpineManager};

fn main() {
    let v = fs::read_to_string("/Users/pontjho/Documents/development/afro-minions/assets/models/majaivan/majaivan.json").unwrap();
    let spine_parser = ConcreteSpineParser {};
    let model = spine_parser.parse(&v).unwrap();

    let spine_manager = ConcreteSpineManager {};
    let attachments = spine_manager.get_attachments_at(0.0, &model, "default");

    println!("{:?}", attachments);
}
