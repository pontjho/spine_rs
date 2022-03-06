use std::fs;
use spine_rs::spine_parser::{ConcreteSpineParser, SpineParser};

fn main() {
    let v = fs::read_to_string("/Users/pontjho/Projects/impi/games/soweto-brawl/assets/models/majaivan/majaivan.json").unwrap();
    let spine_parser = ConcreteSpineParser {};
    let data = spine_parser.parse(&v);

    println!("{:?}", data);
}
