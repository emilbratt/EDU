use std::process;

pub fn run() {
    if let Err(e) = example_error(true) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}

fn example_error(is_ok: bool) -> Result<String, String> {
    if is_ok {
        Ok(String::from("OK"))
    } else {
        Err(String::from("Error"))
    }
}
