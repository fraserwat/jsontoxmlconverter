use serde_json::{self, Map, Value};
use std::error::Error;
use std::io::Cursor;
use xml::writer::{EmitterConfig, EventWriter, XmlEvent};

// For readability's sake instead of having to repeatedly write out the Xml event type object
type XmlCursor = EventWriter<Cursor<Vec<u8>>>;
type ResultGeneric<T> = Result<T, Box<dyn Error>>;

pub fn convert_json_to_xml(json: Value, root_name: &str) -> ResultGeneric<String> {
    // Buffer important for performance reasons. Uses a vector to collect each XML item.
    let buffer = Cursor::new(Vec::new());
    let mut writer = EventWriter::new_with_config(
        buffer,
        // Mandatory config object for the XML writer.
        EmitterConfig::new().perform_indent(true),
    );

    // Start the root element
    writer.write(XmlEvent::start_element(root_name))?;

    // Parse all the JSON
    json_to_xml(&json, &mut writer)?;

    // Close the root element
    writer.write(XmlEvent::end_element())?;

    let xml_bytes = writer
        .into_inner() // Converts Writer back into Cursor
        .into_inner(); // Gets Vec<> object from Cursor
    let xml_string = String::from_utf8(xml_bytes).expect("Should be valid UTF-8");
    Ok(xml_string)
}

fn json_to_xml(json: &Value, writer: &mut XmlCursor) -> ResultGeneric<()> {
    match json {
        Value::String(s) => writer.write(XmlEvent::characters(s)).map_err(|e| e.into()),
        Value::Number(_) | Value::Bool(_) | Value::Null => xml_standard_type_handling(json, writer),
        Value::Array(a) => xml_array_handling(a, writer).map_err(|e| e.into()),
        Value::Object(map) => handle_json_object(map, writer).map_err(|e| e.into()),
        _ => Err("Unsupported JSON value type".to_string()).map_err(|e| e.into()),
    }
}

fn xml_standard_type_handling(json: &Value, writer: &mut XmlCursor) -> ResultGeneric<()> {
    let json_str = match json {
        Value::Null => String::from("null"),
        // All other values reaching this will either be Number or Bool types.
        _ => json.to_string(),
    };
    writer
        .write(XmlEvent::characters(json_str.as_str()))
        .unwrap();
    Ok(())
}

fn xml_array_handling(arr: &[Value], writer: &mut XmlCursor) -> ResultGeneric<()> {
    for item in arr {
        writer.write(XmlEvent::start_element("item"))?;
        json_to_xml(item, writer)?;
        writer.write(XmlEvent::end_element())?;
    }
    Ok(())
}

fn handle_json_object(map: &Map<String, Value>, writer: &mut XmlCursor) -> ResultGeneric<()> {
    for (key, value) in map {
        writer.write(XmlEvent::start_element(key.as_str()))?;
        json_to_xml(value, writer)?;
        writer.write(XmlEvent::end_element())?;
    }
    Ok(())
}

// TESTING BELOW HERE:::

#[cfg(test)]
mod tests {
    use crate::helper::path::construct_file_path;
    use crate::json::read;
    use std::fs;

    fn testing_helper_fn(filename: &str) -> (String, String) {
        let json_path = construct_file_path(&format!("src/data/{}.json", filename));
        let xml_path = construct_file_path(&format!("src/data/{}.xml", filename));

        // Read JSON and convert to XML
        let json = read::load_json(&json_path).unwrap();
        let xml = super::convert_json_to_xml(json, "store").unwrap();

        // Compare to the test XML we have saved
        let xml_test = fs::read_to_string(xml_path).expect("Failed to read Test XML.");
        return (xml, xml_test);
    }

    #[test]
    fn test_trf_simple_xml() {
        let (xml, xml_test) = testing_helper_fn("test_simple");
        assert_eq!(xml, xml_test);
    }
    #[test]
    fn test_trf_nested_xml() {
        let (xml, xml_test) = testing_helper_fn("test_nested");
        assert_eq!(xml, xml_test);
    }
    #[test]
    fn test_trf_mixed_xml() {
        let (xml, xml_test) = testing_helper_fn("test_mixed_data");
        assert_eq!(xml, xml_test);
    }
}
