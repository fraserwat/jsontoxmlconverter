use json::{read, transform};

mod helper;
mod json;

fn main() {
    // Placeholder hardcoding filepath while building
    use crate::helper::path::construct_file_path;
    let file_path = construct_file_path("src/data/test_mixed_data.json");
    let json_result = read::load_json(&file_path).unwrap();

    let xml_result = transform::convert_json_to_xml(json_result, "store");

    println!("{}", xml_result.ok().unwrap())
}
