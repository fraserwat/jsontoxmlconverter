use json::{read as JsonRead, transform as JsonTransform};
use xml::{read as XmlRead, transform as XmlTransform};
mod helper;
mod json;
mod xml;

fn main() {
    // Placeholder hardcoding filepath while building
    use crate::helper::path::construct_file_path;
    let file_path = construct_file_path("src/data/test_simple.xml");
    // let json_input = JsonRead::load_json(&file_path).unwrap();
    // let xml_output = JsonTransform::convert_json_to_xml(json_input, "store");
    // println!("{}", xml_output.ok().unwrap());

    // Placeholder for XML -> JSON Conversion
    // let xml_input = XmlRead::load_xml(file_path);
}
