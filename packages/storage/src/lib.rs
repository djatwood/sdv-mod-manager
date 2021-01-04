use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::ErrorKind;
use std::io::Read;

#[derive(Debug)]
pub struct DB {
    filepath: String,
    config: Config,
    mods: Vec<Manifest>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Config {
    active: String,
    lists: HashMap<String, Vec<String>>,
}

#[serde(rename_all = "PascalCase")]
#[derive(Serialize, Deserialize, Debug)]
struct Manifest {
    name: String,
    author: String,
    version: String,
    minimum_api_version: String,
    description: String,
    #[serde(alias = "UniqueID")]
    unique_id: String,
}

pub fn open(filename: String) -> Result<DB, String> {
    let mut db = DB::new();
    db.filepath = filename;

    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(&db.filepath);
    let mut file = match file {
        Err(e) => return Err(format!("Failed to open file {}: {}", &db.filepath, e)),
        Ok(f) => f,
    };

    let mut contents = String::new();
    match file.read_to_string(&mut contents) {
        Err(e) => return Err(format!("Failed to read file {}: {}", &db.filepath, e)),
        Ok(_) => (),
    }

    if contents.len() > 0 {
        db.config = match serde_json::from_str(&contents) {
            Err(e) => return Err(format!("Failed to parse file {}: {}", &db.filepath, e)),
            Ok(c) => c,
        };
    }
    match db.write() {
        Err(e) => return Err(format!("Failed to write to file {}: {}", &db.filepath, e)),
        Ok(()) => (),
    }

    let paths = match create_or_open_dir("mods") {
        Ok(d) => d,
        Err(e) => return Err(format!("Faild to create mods folder: {}", e)),
    };
    for path in paths {
        let path = match path {
            Err(e) => {
                println!("Failed to parse mod path {}", e);
                continue;
            }
            Ok(p) => p,
        };

        // TODO: Make this cross platform
        let manifest = match File::open(format!("{}/manifest.json", path.path().display())) {
            Err(e) => {
                println!("Failed to open mod {}", e);
                continue;
            }
            Ok(m) => m,
        };

        let manifest: Manifest = match serde_json::from_reader(manifest) {
            Err(e) => {
                println!("Failed to parse manifest file {}", e);
                continue;
            }
            Ok(m) => m,
        };
        db.mods.push(manifest)
    }

    Ok(db)
}

impl DB {
    pub fn new() -> DB {
        DB {
            filepath: String::from(""),
            config: Config::new(),
            mods: Vec::new(),
        }
    }

    pub fn get(&self, name: &str) -> Option<&Vec<String>> {
        self.config.lists.get(name)
    }

    pub fn set(&mut self, name: &str, value: Vec<String>) {
        self.config.lists.insert(name.to_string(), value);
    }

    pub fn write(&self) -> Result<(), String> {
        let file = match OpenOptions::new().write(true).open(&self.filepath) {
            Err(e) => return Err(format!("Failed to open {} : {}", &self.filepath, e)),
            Ok(f) => f,
        };

        match serde_json::to_writer_pretty(file, &self.config) {
            Err(e) => return Err(format!("Failed to format data: {}", e)),
            Ok(_) => return Ok(()),
        }
    }
}

impl Config {
    pub fn new() -> Config {
        let mut lists = HashMap::new();
        lists.insert(String::from("default"), Vec::new());
        Config {
            active: String::from("default"),
            lists: lists,
        }
    }
}

fn create_or_open_dir(p: &str) -> Result<fs::ReadDir, String> {
    match fs::create_dir(p) {
        Ok(()) => (),
        Err(e) => {
            if e.kind() != ErrorKind::AlreadyExists {
                return Err(format!("{}", e));
            }
        }
    }

    match fs::read_dir(p) {
        Ok(d) => Ok(d),
        Err(e) => return Err(format!("{}", e)),
    }
}
