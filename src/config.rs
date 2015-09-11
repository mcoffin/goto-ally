use std::collections::HashMap;
use std::path::{Path, PathBuf};

const CONFIG_FILENAME: &'static str = ".goto.yml";

pub struct Config {
    pub aliases: HashMap<String, PathBuf>,
}

impl Config {
    pub fn new<P>(path: P) -> Config
        where P: AsRef<Path>
    {
        use std::env;
        use std::fs;
        let path = path.as_ref();

        let mut cfg = Config {
            aliases: HashMap::new(),
        };

        // If the path is a file, set the initial path to its parent directory
        let mut path = if fs::metadata(path).unwrap().is_file() {
            path.parent().unwrap()
        } else {
            path
        };

        loop {
            let cfg_path = path.join(CONFIG_FILENAME);
            cfg.apply_config_file(&cfg_path);
            path = match path.parent() {
                Some(p) => p,
                None => break,
            };
        }
        cfg.apply_config_file(env::home_dir().unwrap().join(CONFIG_FILENAME));
        cfg
    }

    fn apply_config_file<P>(&mut self, cfg_path: P)
        where P: AsRef<Path>
    {
        use std::fs;

        let cfg_path = cfg_path.as_ref();
        let cfg_base_path = cfg_path.parent().unwrap();

        match fs::File::open(&cfg_path) {
            Ok(mut f) => {
                use std::io::Read;
                use yaml::yaml::Yaml;
                use yaml::YamlLoader;

                let mut cfg_str = String::new();
                match f.read_to_string(&mut cfg_str) {
                    Err(e) => panic!("Failed to read '{:?}': {}", &cfg_path, e),
                    _ => {},
                }

                let loaded_cfg = YamlLoader::load_from_str(cfg_str.as_ref());
                let cfg_yml = match loaded_cfg {
                    Ok(mut y) => match y.pop() {
                        Some(y) => y,
                        None => panic!("Empty config file at '{:?}'", cfg_path),
                    },
                    Err(e) => panic!("Failed to load config file at '{:?}': {}", cfg_path, e),
                };

                let cfg_yml = match cfg_yml {
                    Yaml::Hash(h) => h,
                    _ => panic!("Malformatted config file at '{:?}': Top element is not hash"),
                }.into_iter();

                let cfg_yml: HashMap<String, PathBuf> = cfg_yml.filter(|t| match t {
                    &(Yaml::String(ref k), Yaml::String(..)) => if self.aliases.contains_key(k) {
                        false
                    } else {
                        true
                    },
                    _ => false,
                }).map(|t| match t {
                    (Yaml::String(ref k), Yaml::String(ref v)) => {
                        let k = k.clone();
                        let v: &Path = v.as_ref();
                        let v = if v.is_absolute() {
                            PathBuf::from(v)
                        } else {
                            cfg_base_path.join(v)
                        };
                        (k, v)
                    },
                    _ => panic!("This is a bug. Please report it. Code: 1"),
                }).collect();

                self.aliases.extend(cfg_yml.into_iter());
            },
            _ => {},
        }
    }
}
