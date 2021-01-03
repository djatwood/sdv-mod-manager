use glob::glob;
use std::env;
use std::fs;
use std::fs::File;
use std::io::ErrorKind;
use std::path::Path;
use storage;

const CONFIGLOCATION: &str = "./config.json";

fn main() {
    let db = storage::open(String::from(CONFIGLOCATION)).unwrap();
    let args: Vec<String> = env::args().collect();

    dbg!(&db);

    match args[1].as_str() {
        "import" => import(&args[2..]),
        "mods" => list_mods(db, args.get(2)),
        _ => println!("TODO: HELP MENU"),
    }
}

fn list_mods(db: storage::DB, name: Option<&String>) {
    let kind = match name {
        None => "default",
        Some(k) => k,
    };

    let list = match db.get(&kind) {
        None => return println!("No modlist named {}", kind),
        Some(l) => l,
    };

    println!("Modlist {} has {} mods", kind, list.len());
    for m in list {
        println!("  {}", m);
    }
}

fn import(paths: &[String]) {
    for p in paths {
        let file_name = match Path::new(p).file_name() {
            Some(f) => match f.to_str() {
                Some(f) => f,
                None => p,
            },
            None => p,
        };

        let dir = match File::open(p) {
            Ok(f) => f,
            Err(e) => {
                println!("Failed to import mod {}: {}", file_name, e);
                continue;
            }
        };
        let is_dir = match dir.metadata() {
            Ok(m) => m.file_type().is_dir(),
            Err(e) => {
                println!("Failed to read metadata on {}: {}", file_name, e);
                continue;
            }
        };
        if !is_dir {
            println!("{} is not a directory", file_name);
            continue;
        }

        println!("Importing {}", file_name);
        match copy_dir(p) {
            Ok(()) => (),
            Err(e) => println!("{}", e),
        };
    }
}

fn copy_dir(dir: &str) -> Result<(), String> {
    let dir = Path::new(dir);
    let dir_name = match dir.file_stem().unwrap().to_str() {
        Some(n) => n,
        None => return Err(format!("Failed to parse name")),
    };
    match fs::create_dir(format!("./mods/{}", dir_name)) {
        Ok(()) => (),
        Err(e) => {
            if e.kind() != ErrorKind::AlreadyExists {
                return Err(format!("Cannot create directory: {}", e));
            }
        }
    }

    let paths = match glob(format!("{}/**/*", dir.display()).as_str()) {
        Ok(p) => p,
        Err(e) => {
            return Err(format!("Failed to read children: {}", e));
        }
    };

    for path in paths {
        let path = match path {
            Ok(path) => path,
            Err(e) => {
                println!("  Failed to parse path: {}", e);
                continue;
            }
        };

        let relative = match Path::new(path.as_path()).strip_prefix(&dir) {
            Ok(path) => path,
            Err(e) => {
                println!("  Failed to parse path {}: {}", path.display(), e);
                continue;
            }
        };

        println!("  {}/{}", dir_name, relative.display());

        if path.is_dir() {
            let name = format!("./mods/{}/{}", dir_name, relative.display());
            match fs::create_dir_all(name) {
                Ok(()) => (),
                Err(e) => {
                    if e.kind() != ErrorKind::AlreadyExists {
                        return Err(format!("Failed to create sub directory: {}", e));
                    }
                }
            }
            continue;
        }

        let location = format!("./mods/{}/{}", dir_name, relative.display());
        match File::create(location.as_str()) {
            Ok(_) => (),
            Err(e) => {
                if e.kind() != ErrorKind::AlreadyExists {
                    println!("  Failed to create file {}: {}", relative.display(), e);
                    continue;
                }
            }
        };

        match fs::copy(&path, location) {
            Ok(_) => (),
            Err(e) => {
                return Err(format!("  Failed to copy file: {}", e));
            }
        };
    }
    Ok(())
}
