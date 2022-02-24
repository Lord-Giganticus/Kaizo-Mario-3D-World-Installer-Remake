extern crate glob;
extern crate zip_extract;
mod rules;
use rules::Rules;
use zip_extract::*;
use glob::*;
use std::*;
use std::path::*;
use std::fs::*;
use std::io::*;


fn main() {
    let envargs: Vec<String> = env::args().collect();
    let fakeargs = &envargs[1..];
    let mut args: Vec<String> = Vec::new();
    for i in 0..fakeargs.len() {
        let arg = &fakeargs[i];
        args.push(String::from(arg));
    }
    let res = glob("..\\downloader\\*.zip").unwrap();
    for item in res {
        let mut current = env::current_dir().unwrap();
        let path = match item {
            Ok(good) => good,
            Err(e) => panic!("{}", e)
        };
        let name = String::from(path.file_stem().unwrap().to_str().unwrap());
        current.push(path.file_stem().unwrap());
        let fullpath = fs::canonicalize(path).unwrap();
        let mut stream = File::open(fullpath).unwrap();
        let mut bytes: Vec<u8> = Vec::new();
        stream.read_to_end(&mut bytes).unwrap();
        let reader = Cursor::new(bytes);
        let dir = Path::new(&current);
        extract(reader, dir, false).unwrap();
        let mut path = String::new();
        path.push_str("Super Mario 3D World/Mods/");
        path.push_str(&name);
        let desc = String::from("Mario's back and this time, I don't think he's gonna have it so easy...");
        let ver: u8 = 5;
        let rules = Rules::new(&args, &name, &path, &desc, &ver);
        let data = format!("{}", rules);
        let mut buf = data.as_bytes();
        let foldername: String;
        if name.contains("practice") {
            foldername = String::from("Kaizo Mario 3D World Practice Mode");
        } else {
            foldername = String::from("Kaizo Mario 3D World");
        }
        let mut location = current.clone();
        location.push(&foldername);
        location.push("rules.txt");
        let mut writer = File::create(location).unwrap();
        writer.write_all(&mut buf).unwrap();
        drop(writer);
        let mut meta = current.clone();
        meta.push(&foldername);
        meta.push("meta");
        meta.push("meta.xml");
        fs::remove_file(meta).unwrap();
        let variant: String;
        if name.contains("practice") {
            variant = String::from("Practice")
        } else {
            variant = String::from("Normal");
        }
        let mut search = String::from("*");
        search.push_str(&variant);
        search.push_str("*.nsi");
        let files = glob(&search).unwrap();
        for file in files {
            let truefile = file.unwrap();
            process::Command::new("cp")
            .arg(truefile).arg(&current)
            .spawn().unwrap().wait().unwrap();
        }
        search = String::from(&variant);
        search.push_str(".cmd");
        process::Command::new("cmd")
        .arg("/c").arg(&search).arg(&current)
        .spawn().unwrap().wait().unwrap();
        fs::remove_dir_all(&current).unwrap();
    }
    process::Command::new("makensis")
    .arg("KM3DW.nsi").spawn().unwrap()
    .wait().unwrap();
}
