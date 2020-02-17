extern crate quick_xml;
extern crate serde;
extern crate serde_json;

use quick_xml::events::{BytesEnd, BytesStart, BytesText, Event};
use quick_xml::Writer;
use serde_json::Value;

use std::io::Cursor;
use std::io::Write;
use std::string::FromUtf8Error;

pub fn convert_json_xml(data: &str) -> Result<String, FromUtf8Error> {
    let value: Value = serde_json::from_str(data).unwrap();
    let mut writer = Writer::new_with_indent(Cursor::new(Vec::new()), ' ' as u8, 2);
    convert_json_key_value_xml("Json", &value, &mut writer);
    let result = writer.into_inner().into_inner();
    Ok(String::from_utf8(result).unwrap())
}

fn convert_json_key_value_xml<W: Write>(key: &str, value: &Value, writer: &mut Writer<W>) {
    let elem = BytesStart::borrowed_name(key.as_bytes());
    writer.write_event(Event::Start(elem)).unwrap();

    convert_json_xml_with_writer(value, writer);

    let elem = BytesEnd::borrowed(key.as_bytes());
    writer.write_event(Event::End(elem)).unwrap();
}

fn convert_json_xml_with_writer<W: Write>(value: &Value, writer: &mut Writer<W>) {
    match value {
        Value::Null => println!("null"),
        Value::Bool(b) => write_text(&format!("{}", b), writer),
        Value::Number(n) => write_text(&format!("{}", n), writer),
        Value::String(s) => write_text(s, writer),
        Value::Array(a) => {
            for (n, v) in a.iter().enumerate() {
                convert_json_key_value_xml(&format!("{}", n), v, writer);
            }
        }
        Value::Object(o) => {
            for (k, v) in o.iter() {
                convert_json_key_value_xml(k, v, writer);
            }
        }
    }
}

fn write_text<W: Write>(text: &str, writer: &mut Writer<W>) {
    let elem = BytesText::from_plain_str(text);
    writer.write_event(Event::Text(elem)).unwrap();
}
