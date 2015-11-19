extern crate getopts;
extern crate yaml_rust as yaml;

mod config;

use getopts::Options;
use std::path::PathBuf;

trait Search<V> {
    fn search<'a, T: AsRef<str>>(&'a self, T) -> Option<&'a V>;
}

impl Search<PathBuf> for config::Config {
    fn search<'a, T: AsRef<str>>(&'a self, tgt: T) -> Option<&'a PathBuf> {
        let tgt = tgt.as_ref();
        match self.aliases.get(tgt) {
            None => {
                None
            },
            Some(found) => Some(found),
        }
    }
}

fn options() -> Options {
    let mut opts = Options::new();
    opts
}

fn goto_target<S: AsRef<str>>(tgt: S) {
    use std::env;

    let tgt = tgt.as_ref();

    let cfg = config::Config::new(env::current_dir().unwrap());

    let resolved = match cfg.search(tgt) {
        Some(d) => d,
        None => panic!("Failed to resolve directory target '{}'", tgt),
    };
    match env::set_current_dir(resolved) {
        Err(e) => panic!("Failed to go to target '{}': {}", tgt, e),
        _ => {},
    }
}

fn print_current_dir() {
    use std::env;

    let to_go = env::current_dir().unwrap();
    let to_go = to_go.to_str().unwrap();
    println!("{}", to_go);
}

fn main() {
    use std::env;

    // Parse command line options
    let opts = options();
    let matches = match opts.parse(env::args().skip(1)) {
        Ok(m) => m,
        Err(e) => panic!("Failed to parse options: {}", e),
    };

    match matches.free.into_iter().next() {
        Some(target) => {
            let tgt_path = target.split("/");
            for tgt_comp in tgt_path {
                goto_target(tgt_comp);
            }
            print_current_dir();
        },
        None => {},
    }
}
