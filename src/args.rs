pub fn get() -> String {
    // We are just expecting a filename for now
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        println!("A filename is expected");
        std::process::exit(1);
    }

    args[1].to_owned()
}
