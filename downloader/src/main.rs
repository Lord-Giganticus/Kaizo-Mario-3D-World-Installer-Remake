extern crate json;
extern crate reqwest;
mod funcs;
use std::collections::*;
use json::JsonValue::{Array, Object};
use std::*;

fn main() {
    let response = reqwest::blocking::get("https://api.gamebanana.com/Core/Item/Data?itemtype=Mod&itemid=149492&fields=Files%28%29.aFiles%28%29").unwrap();
    let data = response.text().unwrap();
    let parsed = json::parse(&data).unwrap();
    let array = match parsed {
        Array(arr) => arr,
        _ => panic!("Bad value.")
    };
    let values = match array.get(0).unwrap() {
        Object(obj) => obj,
        _ => panic!("Bad value.")
    };
    let mut iter = values.iter();
    let mut option = iter.next();
    let mut links = HashMap::new();
    while option != None {
        let obj = match option.unwrap().1 {
            Object(obj) => obj,
            _ => panic!("Bad value.")
        };
        let name = obj.get("_sFile").unwrap().as_str().unwrap();
        let link = obj.get("_sDownloadUrl").unwrap().as_str().unwrap();
        links.insert(name, link);
        option = iter.next();
    }
    let items: Vec<(&str, &str)> = links.into_iter().collect();
    for i in 0..items.len() {
        let item = items.get(i).unwrap();
        let name = item.0;
        let link = item.1;
        if i != items.len() {
            funcs::download(&String::from(link), &String::from(name), false);
        } else {
            funcs::download(&String::from(link), &String::from(name), true);
        }
    }
}