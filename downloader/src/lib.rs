use reqwest::blocking;
use serde::*;
use serde_json::Value as JsonValue;
use std::collections::HashMap;
use std::thread;
use indicatif::*;
pub type NormalResult<T> = Result<T, Box<dyn std::error::Error>>;

#[derive(Deserialize, Debug, Default, Clone)]
pub struct AFile {
    #[serde(rename = "_idRow")]
    pub idrow: i64,
    #[serde(rename = "_sFile")]
    pub sfile: String,
    #[serde(rename = "_nFilesize")]
    pub nfilesize: i64,
    #[serde(rename = "_sDescription")]
    pub sdesc: String,
    #[serde(rename = "_tsDateAdded")]
    pub tsdateadded: i64,
    #[serde(rename = "_nDownloadCount")]
    pub dlcount: i64,
    #[serde(rename = "_sAnalysisState")]
    pub sanalysisstateanalysisstate: String,
    #[serde(rename = "_sDownloadUrl")]
    pub sdownloadurl: String,
    #[serde(rename = "_sMd5Checksum")]
    pub smd5checksum: String,
    #[serde(rename = "_sAnalysisResult")]
    pub sanalysisresult: String,
    #[serde(rename = "_bContainsExe")]
    pub bcontainsexe: bool
}

const URL: &str = "https://api.gamebanana.com/Core/Item/Data?itemtype=Mod&itemid=149492&fields=Files%28%29.aFiles%28%29";

pub fn downloadkaizo() -> NormalResult<Vec<std::path::PathBuf>> {
   let req = blocking::get(URL)?; 
   let val: JsonValue = req.json()?;
   let mut results: HashMap<String, AFile> = HashMap::new();
   if let JsonValue::Array(arr) = val {
    for obj in arr {
        if let JsonValue::Object(obj) = obj {
            for pair in obj {
                results.insert(pair.0, serde_json::from_value(pair.1)?);
            }
        }
    }
   }
   let results = results.iter().map(|(_, file)| file.clone()).collect::<Vec<_>>();
   let mut paths = vec![];
   for i in 0..results.len() {
    if let Some(file) = results.get(i) {
        let last = i >= results.len() - 1;
        let name = &file.sfile;
        let url = &file.sdownloadurl;
        paths.push(download(url, name, last)?);
    }
   }
   Ok(paths)
}

use std::path::*;
use std::fs::*;
use std::io::{Error, ErrorKind};

fn download<S: reqwest::IntoUrl>(link: S, name: S, last: bool) -> NormalResult<PathBuf>  {
    let mut req = blocking::get(link)?;
    let name = name.as_str();
    let mut out = File::create(name)?;
    let prog = ProgressBar::new(req.content_length().unwrap());
    prog.set_style(ProgressStyle::default_bar()
    .template("{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {bytes}/{total_bytes}")?
    .progress_chars("#>-"));
    let t = thread::spawn(move || {
        std::io::copy(&mut req, &mut out)
    });
    while !t.is_finished() {
        let pos = std::fs::metadata(name)?;
        let pos = pos.len();
        prog.set_position(pos);
    }
    let join = t.join();
    prog.set_position(prog.length().unwrap_or_default());
    if last {
        prog.finish();
    }
    match join {
        Ok(res) => match res {
            Ok(_) => Ok(PathBuf::from(name)),
            Err(e) => Err(Box::new(e))
        },
        Err(_) => Err(Box::new(Error::new(ErrorKind::Other, "thread join failed.")))
    }
}