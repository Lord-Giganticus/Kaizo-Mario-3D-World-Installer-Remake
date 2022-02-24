mod funcs;
extern crate json;
extern crate reqwest;
use reqwest::blocking::*;

const VERSION: f32 = 2.69;

fn main() {
    let builder = Client::builder().user_agent("KM3DW-Updater").build().unwrap();
    let resp = builder.get("https://raw.githubusercontent.com/Lord-Giganticus/Kaizo-Mario-3D-World-Installer-Remake/main/version.txt");
    let data = resp.send().unwrap().text().unwrap();
    let ver = data.parse::<f32>().unwrap();
    if ver < VERSION {
        println!("Downloading latest installer and updater....");

    } else {
        println!("No need to update.");
    }
}

fn update() {
    
}