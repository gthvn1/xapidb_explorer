pub fn get() -> Result<String, String> {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        Err("expected a filename for the XAPI DB.".to_string())
    } else {
        Ok(args[1].to_owned())
    }
}
