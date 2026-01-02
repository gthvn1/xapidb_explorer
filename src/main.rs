mod app;

use app::App;

use xapidb_explorer::xapidb::parser::DbNode;

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;

    let root = DbNode::read_xml();
    root.print_tree();

    let terminal = ratatui::init();
    let ret = App::default().run(terminal);
    ratatui::restore();

    ret
}
