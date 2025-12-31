use quick_xml::events::Event;
use quick_xml::reader::Reader;
use std::collections::BTreeMap;
use std::{fs::File, io::Read};

// We want to end with a structure like
//
// name: root
// attributes : []
// children:
//    +--> name: table
//         attributes: "name" = "Cluster"
//         children: []
//    +--> name: table
//         attributes: "name" = Driver_variant"
//         children:
//             +--> name: row
//                  attributes: ["ref" = "...", "__ctime = ...", ...]
//             ...

#[derive(Debug)]
pub struct DbNode {
    pub name: String,
    pub attributes: BTreeMap<String, String>,
    pub children: Vec<DbNode>,
}

pub fn read_xml() -> DbNode {
    let fname = "examples/xapi-db.xml";

    let mut file = match File::open(fname) {
        Err(e) => panic!("failed to open {:?}: {}", fname, e),
        Ok(file) => file,
    };

    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("failed to read contents");

    let mut reader = Reader::from_str(&contents);
    reader.config_mut().trim_text(true);

    let mut buf = Vec::new();

    let mut stack: Vec<DbNode> = vec![DbNode {
        name: "root".to_string(),
        attributes: BTreeMap::new(),
        children: Vec::new(),
    }];

    loop {
        match reader.read_event_into(&mut buf) {
            Err(e) => panic!("Error at position {}: {:?}", reader.error_position(), e),
            Ok(Event::Eof) => break,
            Ok(Event::Start(e)) => {
                // Start tag (with attributes) <tag attr="value">
                println!("---- [start] begin ");
                let name = str::from_utf8(e.name().as_ref()).unwrap().to_string();
                let mut attrs = BTreeMap::new();

                for attr in e.attributes() {
                    let attr = attr.unwrap();
                    let key_bytes = attr.key.as_ref();
                    let key = str::from_utf8(key_bytes).expect("invalid utf8 key");
                    let value = attr
                        .unescape_value()
                        .expect("failed to unescape value")
                        .into_owned();
                    attrs.insert(key.to_string(), value.to_string()).unwrap();
                }
                match name.as_str() {
                    "table" => {
                        // We are expecting to have only root in stack
                        assert!(stack.len() == 1);

                        // push it on the stack
                        stack.push(DbNode {
                            name: "table".to_string(),
                            attributes: attrs,
                            children: Vec::new(),
                        });
                    }
                    "row" => {
                        let row = DbNode {
                            name: "row".to_string(),
                            attributes: attrs,
                            children: Vec::new(),
                        };

                        // Attach the row to the last node of the stack
                        stack.last_mut().unwrap().children.push(row);
                    }
                    "manifest" | "database" => {}
                    name => unreachable!("node {} is not expected", name),
                }
                dbg!(&e);
                println!("---- [start] end");
            }
            Ok(Event::End(_)) => {
                // End tag </tag>
                stack.pop();
            }
            Ok(Event::Empty(e)) => {
                // Empty element tag (with attributes) <tag attr="value" />
                let name = str::from_utf8(e.name().as_ref())
                    .expect("invald utf8 name")
                    .to_string();
                println!("---- [empty] begin -> {}", name);
                if name == "table" {
                    let node = stack.pop().unwrap();
                    // Add the node to the last node of the stack that we are expecting to be root
                    assert!(stack.len() == 1);
                    stack.last_mut().unwrap().children.push(node);
                }
                println!("---- [empty] end");
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

    assert!(stack.len() == 1);
    let root = stack.pop().unwrap();
    dbg!(&root);
    root
}
