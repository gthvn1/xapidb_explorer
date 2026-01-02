mod app;

use app::App;

use xapidb_explorer::xapidb::parser::DbNode;

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;

    let root = DbNode::read_xml();
    dbg!(&root);

    let terminal = ratatui::init();
    let ret = App::new(root).run(terminal);
    ratatui::restore();

    ret
}
