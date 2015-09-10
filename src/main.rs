extern crate getopts;
extern crate yaml_rust as yaml;

mod config;

use getopts::Options;

fn options() -> Options {
    let mut opts = Options::new();
    opts
}

fn main() {
    use std::env;

    // Parse command line options
    let opts = options();
    let matches = match opts.parse(env::args().skip(1)) {
        Ok(m) => m,
        Err(e) => panic!("Failed to parse options: {}", e),
    };

    // Load configuration
    let cfg = config::Config::new(env::current_dir().unwrap());

    // Actually go to the target
    for target in &matches.free {
        let to_go = cfg.aliases[target].to_str().unwrap();
        println!("{}", to_go);
        return;
    }
}
