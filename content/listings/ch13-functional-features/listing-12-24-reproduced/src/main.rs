use std::env;
use std::process;

use minigrep::Config;

// ANCHOR: ch13
fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|err| {
        eprintln!("Argumentlarni tahlil qilish muammosi: {err}");
        process::exit(1);
    });

    // --snip--
    // ANCHOR_END: ch13

    if let Err(e) = minigrep::run(config) {
        eprintln!("Dastur xatosi: {e}");
        process::exit(1);
    }
    // ANCHOR: ch13
}
// ANCHOR_END: ch13
