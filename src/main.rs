use std::env;
use storage;

const CONFIGLOCATION: &str = "./config.json";

fn main() {
    let db = storage::open(String::from(CONFIGLOCATION)).unwrap();
    let args: Vec<String> = env::args().collect();

    match args[1].as_str() {
        "import" => import(&args[2]),
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

fn import(md: &String) {
    println!("import {}", md)
}
