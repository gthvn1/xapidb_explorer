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

impl DbNode {
    pub fn read_xml(fname: &str) -> DbNode {
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
                    // We are only interesting by <table>
                    let name = str::from_utf8(e.name().as_ref()).unwrap().to_string();
                    if name == "table" {
                        // A table has only a name as attribute
                        let attr = e.attributes().next().unwrap().unwrap();
                        let key_bytes = attr.key.as_ref();
                        let key = str::from_utf8(key_bytes).expect("invalid utf8 key");
                        let value = attr
                            .unescape_value()
                            .expect("failed to unescape value")
                            .into_owned();

                        // We can add table in vec
                        stack.push(DbNode {
                            name: "table".to_string(),
                            attributes: BTreeMap::from([(key.to_string(), value)]),
                            children: Vec::new(),
                        });
                    }
                }
                Ok(Event::End(e)) => {
                    // End tag </tag>
                    // As we are only pushing table on the stack, we are also only popping table
                    let name = str::from_utf8(e.name().as_ref()).unwrap().to_string();
                    if name == "table" {
                        let node = stack.pop().unwrap();
                        // Add the node to the last node of the stack that we are expecting to be root
                        assert!(stack.len() == 1);
                        stack.last_mut().unwrap().children.push(node);
                    }
                }
                Ok(Event::Empty(e)) => {
                    // Empty element tag (with attributes) <tag attr="value" />
                    // Here we need to just add table to root and row to attribute table
                    let name = str::from_utf8(e.name().as_ref()).unwrap().to_string();
                    let mut attributes = BTreeMap::new();

                    for attr in e.attributes() {
                        let attr = attr.unwrap();
                        let key_bytes = attr.key.as_ref();
                        let key = str::from_utf8(key_bytes).expect("invalid utf8 key");
                        let value = attr
                            .unescape_value()
                            .expect("failed to unescape value")
                            .into_owned();
                        attributes.insert(key.to_string(), value);
                    }

                    //dbg!(&e);

                    match name.as_str() {
                        "table" => {
                            // We can add table in root
                            // Here we are expecting only root in stack
                            assert!(stack.len() == 1);
                            stack.last_mut().unwrap().children.push(DbNode {
                                name: "table".to_string(),
                                attributes,
                                children: Vec::new(),
                            });
                        }
                        "row" => {
                            // Here we are expecting root AND a table in stack
                            // Create a row node with all attributes and add it as children of the table
                            assert!(stack.len() == 2);
                            stack.last_mut().unwrap().children.push(DbNode {
                                name: "row".to_string(),
                                attributes,
                                children: Vec::new(),
                            });
                        }
                        _ => {} // ignore everything else
                    }
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

        // If everything goes well the root is the last node in the stack.
        assert!(stack.len() == 1);
        stack.pop().unwrap()
    }

    pub fn has_children(&self) -> bool {
        !self.children.is_empty()
    }

    // It returns the name of the DbNode that can be either a name found in
    // attributes if it is a "row" or the name in case of table that doesn't have
    // name in attributes.
    pub fn get_name(&self) -> String {
        if let Some(name) = self.attributes.get("name") {
            name.to_string()
        } else if let Some(name) = self.attributes.get("name__label") {
            name.to_string()
        } else if let Some(name) = self.attributes.get("ref") {
            name.to_string()
        } else {
            self.name.to_string()
        }
    }

    pub fn print_tree(&self) {
        for (idx, child) in self.children.iter().enumerate() {
            // Check if there is a name in attributes
            assert!(child.name == "table");
            match child.attributes.get("name") {
                None => println!("{}:{}", idx, child.name),
                Some(v) => println!("{}: {}", idx, v),
            }

            // And print its grand children (a row)
            for (i, gchild) in child.children.iter().enumerate() {
                // Check if there is a name in attributes
                assert!(gchild.name == "row");
                println!("  {}: row", i);
                for (k, v) in gchild.attributes.iter() {
                    println!("    {k}: {v}");
                }
            }
        }
    }
}
