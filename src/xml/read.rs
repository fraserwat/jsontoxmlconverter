use std::collections::VecDeque;
use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use xml::reader::{EventReader, XmlEvent};

type Reader = EventReader<BufReader<File>>;
type ResultGnr<T> = Result<T, Box<dyn Error>>;
type XmlAttribute = [xml::attribute::OwnedAttribute];

pub fn load_xml(file_path: &str) -> ResultGnr<Reader> {
    let file = File::open(file_path)?;
    let file_reader = BufReader::new(file);
    let xml_input = EventReader::new(file_reader);
    match check_xml_parse(xml_input) {
        // Previous file_reader consumed by Rust's ownership system, so just rerun & return the above:
        Ok(_) => Ok(EventReader::new(BufReader::new(File::open(file_path)?))),
        Err(error) => Err(error),
    }
}

// TODO: THis could be cleaned up by creating an XMLError enum type.

fn check_xml_parse(parser: Reader) -> ResultGnr<String> {
    // xml-rs library only really checks if its able to be opened, no structural checks, so check for:
    // // 1. Naming conventions -- tag names cannot have spaces in xml.
    // // 2. Structural conventions -- any opened tag needs a closing tag, we'll use a stack for this.
    let mut tag_stack: VecDeque<String> = VecDeque::new();
    let mut reconstructed_xml = String::new();

    for line in parser {
        match line {
            // Checking for 1. Naming conventions.
            Ok(XmlEvent::StartElement {
                name, attributes, ..
            }) => {
                if name.local_name.contains(" ") {
                    return Err(format!("Elem '{}' cannot contain spaces", name.local_name).into());
                }
                // Push open tag to stack (for checking later) and successfully parsed line to new XML.
                tag_stack.push_back(name.local_name.clone());
                xml_start_tag(&mut reconstructed_xml, &name.local_name, &attributes)?;
            }
            // Checking for 2. Structural conventions.
            Ok(XmlEvent::EndElement { name }) => {
                if Some(&name.local_name) != tag_stack.pop_back().as_ref() {
                    return Err(
                        format!("Element '{}' closing tag missing.", name.local_name).into(),
                    );
                }
                xml_end_tag(&mut reconstructed_xml, &name.local_name)?;
            }
            Ok(XmlEvent::Characters(data)) => reconstructed_xml.push_str(&data),
            Err(error) => return Err(error.into()),
            _ => {}
        }
    }
    if !tag_stack.is_empty() {
        return Err("Tags left unclosed in stack -- XML is missing a closing tag.".into());
    }
    Ok(reconstructed_xml)
}

fn xml_start_tag(output: &mut String, tag_name: &str, attributes: &XmlAttribute) -> ResultGnr<()> {
    output.push('<');
    output.push_str(tag_name);
    attributes.iter().for_each(|attr| {
        output.push(' ');
        output.push_str(&attr.name.local_name);
        output.push_str("=\"");
        output.push_str(&attr.value);
        output.push('"');
    });
    output.push('>');
    Ok(())
}

fn xml_end_tag(output: &mut String, tag_name: &str) -> ResultGnr<()> {
    output.push_str("</");
    output.push_str(tag_name);
    output.push('>');
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::helper::path::construct_file_path;

    #[test]
    fn test_read_broken_naming() {
        let file_path = construct_file_path("src/data/test_broken_naming.xml");
        assert!(load_xml(&file_path).is_err());
    }

    #[test]
    fn test_read_broken_tag() {
        let file_path = construct_file_path("src/data/test_broken_tag.xml");
        assert!(load_xml(&file_path).is_err());
    }

    #[test]
    fn test_read_simple() {
        let file_path = construct_file_path("src/data/test_simple.xml");
        assert!(load_xml(&file_path).is_ok())
    }

    #[test]
    fn test_read_nested() {
        let file_path = construct_file_path("src/data/test_nested.xml");
        assert!(load_xml(&file_path).is_ok());
    }

    #[test]
    fn test_read_mixed_data() {
        let file_path = construct_file_path("src/data/test_mixed_data.xml");
        assert!(load_xml(&file_path).is_ok());
    }
}
