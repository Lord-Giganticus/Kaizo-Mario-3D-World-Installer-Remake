mod funcs;
extern crate json;
extern crate reqwest;
use reqwest::blocking::*;
use json::JsonValue::{Array, Object};
use std::env::*;

const VERSION: f32 = 2.69;
const FORCEUPDATE: &str = "--force-update";

fn main() {
    let envargs: Vec<String> = args().collect();
    let args = &envargs[1..];
    let check: String;
    if args.len() != 0 {
        check = String::from(&args[0]);
    } else {
        check = String::new();
    }
    let builder = Client::builder().user_agent("KM3DW-Updater").build().unwrap();
    if check == String::from(FORCEUPDATE) {
        println!("Forcing a immediate download of the lattest assets.");
        update(&builder, get_assets_link(&builder));
        return;
    }
    let resp = builder.get("https://raw.githubusercontent.com/Lord-Giganticus/Kaizo-Mario-3D-World-Installer-Remake/main/version.txt");
    let data = resp.send().unwrap().text().unwrap();
    let ver = data.parse::<f32>().unwrap();
    if ver < VERSION {
        println!("Downloading latest installer and updater....");
        update(&builder, get_assets_link(&builder));
    } else {
        println!("No need to update.");
    }
}

fn get_assets_link(builder: &Client) -> String {
    let link = "https://api.github.com/repos/Lord-Giganticus/Kaizo-Mario-3D-World-Installer-Remake/releases";
    let data = builder.get(link).send().unwrap().text().unwrap();
    let parsed = json::parse(&data).unwrap();
    let array = match parsed {
        Array(arr) => arr,
        _ => panic!("Bad Value.")
    };
    let values = match array.get(0).unwrap() {
        Object(obj) => obj,
        _ => panic!("Bad Value.")
    };
    let res = String::from(values.get("assets_url").unwrap().as_str().unwrap());
    return res;
}

fn update(builder: &Client, assets_link: String) {
    let data = builder.get(assets_link).send().unwrap().text().unwrap();
    let parsed = json::parse(&data).unwrap();
    let array = match parsed {
        Array(arr) => arr,
        _ => panic!("Bad Value.")
    };
    for i in 0..array.len() {
        let values = match array.get(i).unwrap() {
            Object(obj) => obj,
            _ => panic!("Bad Value.")
        };
        let name = values.get("name").unwrap().as_str().unwrap().to_owned();
        let link = values.get("browser_download_url").unwrap().as_str().unwrap().to_owned();
        if i != 1 {
            funcs::download(&link, &name, false);
        } else {
            funcs::download(&link, &name, true);
        }
    }
}