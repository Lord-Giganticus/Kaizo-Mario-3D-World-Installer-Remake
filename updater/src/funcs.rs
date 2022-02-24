extern crate reqwest;
extern crate indicatif;
extern crate thread_control;
use reqwest::*;
use indicatif::*;
use thread_control::*;
use std::fs::*;
use std::*;

pub fn download(arg: &String, name: &String, last: bool) -> String {
    let mut req = blocking::get(arg).unwrap();
    let mut out = File::create(name).unwrap();
    let prog = ProgressBar::new(req.content_length().unwrap());
    prog.set_style(ProgressStyle::default_bar()
    .template("{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {bytes}/{total_bytes}")
    .progress_chars("#>-"));
    let (flag, control) = make_pair();
    let handel = thread::spawn(move || {
        while flag.is_alive() {
            io::copy(&mut req, &mut out).unwrap();
            break;
        }
    });
    while !control.is_done() {
        prog.set_position(fs::metadata(name).unwrap().len());
        if control.is_done() {
            control.stop();
            break;
        }
    }
    handel.join().unwrap();
    prog.set_position(prog.length());
    if last {
        prog.finish();
    }
    return String::from(name);
}