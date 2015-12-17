#![feature(fnbox)]

#[macro_use] extern crate effect_monad;
extern crate getopts;
extern crate yaml_rust as yaml;

mod config;

use effect_monad::EffectMonad;
use getopts::Options;
use std::boxed::FnBox;
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

fn goto_target<'a>(tgt: &'a str) -> Box<FnBox() -> ()>{
    let cfg = config::Config::new(env::current_dir().unwrap());

    let tgt = tgt.to_string();
    Box::new((move || {
        let search_result = cfg.search(&tgt);
        (search_result, tgt)
    }).bind(|(search_result, tgt)| match search_result {
        Some(d) => effect_map!((d, tgt)),
        None => panic!("Failed to resolve directory target '{}'", tgt),
    }).bind(|(resolved, tgt)| {
        effect_map!((env::set_current_dir(resolved), tgt))
    }).bind(|(result, tgt)| {
        effect_map!(match result {
            Err(e) => panic!("Failed to go to target '{}': {}", tgt, e),
            _ => {},
        })
    }))
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
    env::args.bind(|args| {
        effect_map!(options().parse(args.skip(1)))
    }).bind(|parse_result| match parse_result {
        Ok(m) => effect_map!(m),
        Err(e) => panic!("Failed to parse options: {}", e),
    }).bind(|matches| match matches.free.into_iter().next() {
        Some(target) => {
            let tgt_path = target.split("/");
            let empty_effect: Box<FnBox() -> ()> = Box::new(effect_map!(()));
            tgt_path.map(|tgt_comp| goto_target(tgt_comp))
                .fold(empty_effect, |a, b| Box::new(a.bind_ignore_contents(b)))
        },
        None => panic!("You must supply a target"),
    }).bind_ignore_contents(print_current_dir)();
}
