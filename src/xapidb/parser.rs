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
}
