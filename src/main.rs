mod app;
mod args;

use app::App;

use xapidb_explorer::xapidb::parser::DbNode;

fn main() -> color_eyre::Result<()> {
    match args::get() {
        Ok(filename) => {
            let terminal = ratatui::init();
            let root = DbNode::read_xml(&filename);
            let res = App::new(root).run(terminal);
            ratatui::restore();
            res
        }
        Err(e) => {
            println!("Error: {e}");
            std::process::exit(1);
        }
    }
}
