#![allow(unused_variables, unused_mut)]

use quick_xml::events::Event; //BytesStart
use quick_xml::reader::Reader;

fn main() {
    let xml = r#"
    <dochead>
    <grphead>Group Header</grphead>
    <outer ccy=CHF>
        <inner>
        here comes my 1st text
        </inner>
    </outer>
    <outer>
        <inner>
        here comes my 2nd text
        </inner>
    </outer>
    </dochead>
"#;
    let mut reader = Reader::from_str(xml);
    // let mut reader = Reader::from_file("C:/test/test.xml").unwrap();

    reader.trim_text(true);
    let mut elname: String;
    let mut buf = Vec::new();
    let mut start_pos: usize = 0;
    let mut prev_tag_end: usize = 0;
    let mut has_attributes = false;
    let mut attrs: &[u8];
    let mut attrs_len: usize = 0;

    let needle = "outer";

    // The processing of the reader consumes the same, hence create a copy
    // of the same before we start conuming it
    let cursor = reader.clone().into_inner();

    // The `Reader` does not implement `Iterator` because it outputs borrowed data (`Cow`s)
    loop {
        match reader.read_event_into(&mut buf) {
            Ok(Event::Start(ref e)) => {
                // get the element name
                elname = String::from_utf8_lossy(e.name().into_inner()).to_string();
                if elname.eq(needle) {
                    let end = e.to_end();

                    let end_pos = reader.buffer_position();
                    println!("current element is: {elname}");
                    println!(
                        "current position in buffer (end_pos): {end_pos} | start_pos: {start_pos} | prev_end_pos: {prev_tag_end}"
                    );
                    println!("cursor lenght: {:#?}", cursor.len());

                    // TODO!!!
                    // the cursor[start_pos..prev_tag_end] is buggy, this needs correction
                    let prev_content =
                        String::from_utf8(cursor[start_pos..prev_tag_end].to_owned())
                            .expect("can't make string with previous content");
                    println!("Previous content:\n||{prev_content}||\n");

                    let mut tag = reader
                        .read_text(end.name())
                        .unwrap_or(std::borrow::Cow::from("no text"))
                        .to_string();

                    // memorize the position
                    prev_tag_end = reader.buffer_position();

                    // handle attributes
                    for _ in e.attributes() {
                        has_attributes = true;
                        break;
                    }
                    if has_attributes {
                        attrs = e.attributes_raw();
                        let attrs_string = String::from_utf8_lossy(attrs);

                        println!("{}", attrs_string);
                    }
                    tag = format!("<{needle}>{tag}</{needle}>");

                    println!("printing tag: || {tag} ||");
                    start_pos = reader.buffer_position();
                }
            }
            Ok(Event::Text(ref _e)) => {
                // process the Tag Content
            }
            Ok(Event::CData(ref _e)) => {
                // process the Tag Content
            }
            Ok(Event::Empty(_e)) => {} //no need to process empty elements
            Ok(Event::Comment(_e)) => {} //no need to process empty elements
            Ok(Event::Decl(_e)) => {}
            Ok(Event::PI(_e)) => {} //no need to process processing instructions
            Ok(Event::DocType(_e)) => {}
            Ok(Event::End(_e)) => {
                // Tag End reached
                // do cleanup work as needed
                has_attributes = false;

                // memorize the position
                prev_tag_end = reader.buffer_position();
            }
            Ok(Event::Eof) => break, // exits the loop when reaching end of file
            Err(e) => {
                // return an error
                let msg = format!("Error at position {}: {:?}", reader.buffer_position(), e);
                // return Err(msg)?;
                println!("{msg}");
            } // _ => (), // All `Event`s are handled above
        }

        // if we don't keep a borrow elsewhere, we can clear the buffer to keep memory usage low
        buf.clear();
    }
}
