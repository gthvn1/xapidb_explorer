mod app;

use app::App;

use xapidb_explorer::xapidb::parser::DbNode;

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;

    let terminal = ratatui::init();
    // TODO: pass the file to parse as an argument
    let root = DbNode::read_xml();
    let ret = App::new(root).run(terminal);
    ratatui::restore();

    ret
}
