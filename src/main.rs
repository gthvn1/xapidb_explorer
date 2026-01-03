mod app;
mod args;

use app::App;

use xapidb_explorer::xapidb::parser::DbNode;

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    let terminal = ratatui::init();
    let filename = args::get();
    let root = DbNode::read_xml(&filename);
    let ret = App::new(root).run(terminal);
    ratatui::restore();

    ret
}
