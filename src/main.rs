pub use polodb_core::db::Database;
pub use polodb_core::db::DbResult;
use std::env;
use std::fs;

fn main() {
    let mut db = match Database::open("db.json") {
        Err(e) => panic!("Failed to connect to database: {}", e),
        Ok(db) => db,
    };
    let mut store = match db.collection("mods") {
        Err(e) => panic!("Failed to get collection mods: {}", e),
        Ok(store) => store,
    };

    let args: Vec<String> = env::args().collect();

    match args[1].as_str() {
        "import" => import(&args[2]),
        "mods" => {
            let status = if args.len() > 2 {
                String::from(&args[2])
            } else {
                String::new()
            };
            list_mods(&status);
        }
        _ => println!("TODO: HELP MENU"),
    }

    let paths = fs::read_dir("../Mods").unwrap();
    for path in paths {
        println!("Name: {}", path.unwrap().path().display())
    }
}

fn list_mods(kind: &String) {
    match kind.as_str() {
        "enabled" => println!("list enabled mods"),
        "disabled" => println!("list disabled mods"),
        "" => println!("list all mods"),
        _ => println!("TODO: show options"),
    }
}

fn import(md: &String) {
    println!("import {}", md)
}
