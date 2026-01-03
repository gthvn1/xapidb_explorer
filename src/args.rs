pub fn get() -> color_eyre::Result<String> {
    // We are just expecting a filename for now
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        color_eyre::eyre::bail!("A filename is expected");
    }

    Ok(args[1].to_owned())
}
