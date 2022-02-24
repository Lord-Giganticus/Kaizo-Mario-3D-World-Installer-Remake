mod funcs;
extern crate json;
extern crate reqwest;
use reqwest::blocking::*;
use json::JsonValue::{Array, Object};

const VERSION: f32 = 2.69;

fn main() {
    let builder = Client::builder().user_agent("KM3DW-Updater").build().unwrap();
    let resp = builder.get("https://raw.githubusercontent.com/Lord-Giganticus/Kaizo-Mario-3D-World-Installer-Remake/main/version.txt");
    let data = resp.send().unwrap().text().unwrap();
    let ver = data.parse::<f32>().unwrap();
    update(&builder, get_assets_link(&builder));
    if ver < VERSION {
        println!("Downloading latest installer and updater....");
        
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
    let values = match array.get(0).unwrap() {
        Object(obj) => obj,
        _ => panic!("Bad Value.")
    };
    println!("{:?}", values);
}