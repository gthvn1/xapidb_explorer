use xapidb_explorer::xapidb::parser::DbNode;

fn main() {
    let root = DbNode::read_xml();
    root.print_tree();
}
