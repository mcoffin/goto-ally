extern crate getopts;
extern crate yaml_rust as yaml;

mod config;

use getopts::Options;
use std::env;
use std::path::PathBuf;
use std::fs;

trait Search<V> {
    fn search<'a, T: AsRef<str>>(&'a self, T) -> Option<V>;
}

impl Search<PathBuf> for config::Config {
    fn search<'a, T: AsRef<str>>(&'a self, tgt: T) -> Option<PathBuf> {
        let tgt = tgt.as_ref();
        match self.aliases.get(tgt) {
            None => {
                let current_dir = env::current_dir().unwrap();
                for entry in fs::read_dir(&current_dir).unwrap() {
                    match entry {
                        Ok(ent) => {
                            let ent = ent.file_name();
                            if tgt.eq(&ent) {
                                return Some(PathBuf::from(ent));
                            }
                        },
                        _ => {},
                    }
                }
                None
            },
            Some(found) => Some(found.clone()),
        }
    }
}

fn options() -> Options {
    let mut opts = Options::new();
    opts
}

fn goto_target<S: AsRef<str>>(tgt: S) {
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
