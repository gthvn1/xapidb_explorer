use quick_xml::events::Event;
use quick_xml::reader::Reader;
use std::{fs::File, io::Read};

pub fn read_xml() {
    let fname = "examples/xapi-db.xml";

    let mut file = match File::open(fname) {
        Err(e) => panic!("failed to open {:?}: {}", fname, e),
        Ok(file) => file,
    };

    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("failed to read contents");

    let first: String = contents.chars().take(20).collect();
    println!("{}", first);

    let mut reader = Reader::from_str(&contents);
    reader.config_mut().trim_text(true);

    let mut buf = Vec::new();

    loop {
        match reader.read_event_into(&mut buf) {
            Err(e) => panic!("Error at position {}: {:?}", reader.error_position(), e),
            Ok(Event::Eof) => break,
            Ok(Event::Start(e)) => {
                // Start tag (with attributes) <tag attr="value">
                println!("found {:?}", e.name());
            }
            Ok(Event::End(_)) => {
                // End tag </tag>
                println!("found end");
            }
            Ok(Event::Empty(e)) => {
                // Empty element tag (with attributes) <tag attr="value" />
                println!("---- empty begin");
                dbg!(e);
                println!("---- empty end");
            }
            Ok(Event::Decl(_)) => {} // can be skipped
            // Other events are not expected in our DB so just panic
            Ok(Event::Text(_)) => unreachable!("found text"),
            Ok(Event::CData(_)) => unreachable!("found cdata"),
            Ok(Event::Comment(_)) => unreachable!("found comment"),
            Ok(Event::PI(_)) => unreachable!("found pi"),
            Ok(Event::DocType(_)) => unreachable!("found doctype"),
            Ok(Event::GeneralRef(_)) => unreachable!("found generalref"),
        }
    }

    println!("done");
}
