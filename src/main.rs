mod app;
mod args;

use app::App;

use xapidb_explorer::xapidb::parser::DbNode;

struct TerminalDefer;
impl Drop for TerminalDefer {
    fn drop(&mut self) {
        ratatui::restore()
    }
}

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    let terminal = ratatui::init();
    let _defer = TerminalDefer;

    let filename = args::get()?;
    let root = DbNode::read_xml(&filename);
    App::new(root).run(terminal)
}
