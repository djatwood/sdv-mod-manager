use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    match args[1].as_str() {
        "import" => import(&args[2]),
        "mods" => {
            if args.len() < 3 {
                list_mods(&String::new());
            } else {
                list_mods(&args[2]);
            }
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
